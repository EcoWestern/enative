mod state;

use std::{mem::ManuallyDrop, os::fd::AsFd, sync::Arc, time::Duration};

use crate::{
    actions::{LayerShellActions, LayershellCustomActions, LayershellCustomActionsWithInfo},
    clipboard::LayerShellClipboard,
    conversion,
    error::Error,
    settings::VirtualKeyboardSettings,
};

use enative_core::{renderer, window, Event as NativeEvent, Size};
use enative_core::time::Instant;
use enative_futures::{Executor, Runtime, subscription};
use enative_graphics::{Compositor, Shell, compositor};
use enative_program::Program;
use enative_runtime::{Action, task as runtime_task, user_interface, UserInterface};

use state::State;

use layershellev::{
    calloop::timer::{TimeoutAction, Timer},
    reexport::zwp_virtual_keyboard_v1,
    LayerEvent, ReturnData, WindowWrapper,
};

use futures::{channel::mpsc, StreamExt};

use crate::{event::LayershellEvent, proxy::LayershellProxy, settings::Settings};

type SingleRuntime<E, Message> =
    Runtime<E, LayershellProxy<Action<Message>>, Action<Message>>;

pub fn run<P, E, C>(program: P, settings: Settings) -> Result<(), Error>
where
    P: Program + 'static,
    P::Theme: Default,
    E: Executor + 'static,
    C: Compositor<Renderer = P::Renderer> + 'static,
    P::Message: 'static + TryInto<LayershellCustomActions, Error = P::Message>,
{
    use futures::task;
    use futures::Future;

    let (message_sender, message_receiver) = std::sync::mpsc::channel::<Action<P::Message>>();

    let proxy = LayershellProxy::new(message_sender);
    let mut runtime: SingleRuntime<E, P::Message> = {
        let executor = E::new().map_err(Error::ExecutorCreationFailed)?;
        Runtime::new(executor, proxy)
    };

    let (program_state, task) = runtime.enter(|| program.boot());

    let ev = layershellev::WindowStateSimple::new(P::name())
        .with_single(true)
        .with_use_display_handle(true)
        .with_option_size(settings.layer_settings.size)
        .with_layer(settings.layer_settings.layer)
        .with_anchor(settings.layer_settings.anchor)
        .with_exclusize_zone(settings.layer_settings.exclusive_zone)
        .with_margin(settings.layer_settings.margin)
        .with_keyboard_interacivity(settings.layer_settings.keyboard_interactivity)
        .with_xdg_output_name(settings.layer_settings.binded_output_name)
        .build()
        .expect("Cannot create layershell");

    let window = Arc::new(ev.gen_main_wrapper());

    if let Some(stream) = runtime_task::into_stream(task) {
        runtime.run(stream);
    }

    runtime.track(subscription::into_recipes(
        runtime.enter(|| program.subscription(&program_state).map(Action::Output)),
    ));

    let main_id = window::Id::unique();
    let view_state = State::new(&program, &program_state, &ev, main_id);

    let (mut event_sender, event_receiver) =
        mpsc::unbounded::<LayershellEvent<Action<P::Message>, ()>>();
    let (control_sender, mut control_receiver) = mpsc::unbounded::<LayerShellActions<()>>();

    let compositor_settings = compositor::Settings {
        antialiasing: if settings.antialiasing {
            Some(enative_graphics::Antialiasing::MSAAx4)
        } else {
            None
        },
        ..Default::default()
    };

    let renderer_settings = renderer::Settings {
        default_font: settings.default_font,
        default_text_size: settings.default_text_size,
    };

    let mut instance = Box::pin(run_instance::<P, E, C>(
        program,
        program_state,
        compositor_settings,
        renderer_settings,
        runtime,
        event_receiver,
        control_sender,
        view_state,
        window.clone(),
        settings.fonts,
    ));

    let mut context = task::Context::from_waker(task::noop_waker_ref());
    let mut pointer_serial: u32 = 0;

    let _ = ev.running_with_proxy(message_receiver, move |event, ev, _| {
        use layershellev::DispatchMessage;
        let mut def_returndata = ReturnData::None;
        match event {
            LayerEvent::InitRequest => {
                if settings.virtual_keyboard_support.is_some() {
                    def_returndata = ReturnData::RequestBind;
                }
            }
            LayerEvent::BindProvide(globals, qh) => {
                let virtual_keyboard_manager = globals
                    .bind::<zwp_virtual_keyboard_v1::ZwpVirtualKeyboardManagerV1, _, _>(
                        qh,
                        1..=1,
                        (),
                    )
                    .expect("no support virtual_keyboard");
                let VirtualKeyboardSettings {
                    file,
                    keymap_size,
                    keymap_format,
                } = settings.virtual_keyboard_support.as_ref().unwrap();
                let seat = ev.get_seat();
                let virtual_keyboard_in =
                    virtual_keyboard_manager.create_virtual_keyboard(seat, qh, ());
                virtual_keyboard_in.keymap((*keymap_format).into(), file.as_fd(), *keymap_size);
                ev.set_virtual_keyboard(virtual_keyboard_in);
            }
            LayerEvent::RequestMessages(message) => {
                if let DispatchMessage::MouseEnter { serial, .. } = message {
                    pointer_serial = *serial;
                }
                event_sender
                    .start_send(message.into())
                    .expect("Cannot send");
            }
            LayerEvent::NormalDispatch => {
                event_sender
                    .start_send(LayershellEvent::NormalUpdate)
                    .expect("Cannot send");
            }
            LayerEvent::UserEvent(event) => {
                let _ = event_sender
                    .start_send(LayershellEvent::UserEvent(event))
                    .ok();
            }
            _ => {}
        }
        let poll = instance.as_mut().poll(&mut context);
        let task::Poll::Pending = poll else {
            return ReturnData::RequestExit;
        };
        let Ok(flow) = control_receiver.try_recv() else {
            return def_returndata;
        };
        match flow {
            LayerShellActions::CustomActions(action) => match action {
                LayershellCustomActionsWithInfo::AnchorChange(anchor) => {
                    ev.main_window().set_anchor(anchor);
                }
                LayershellCustomActionsWithInfo::LayerChange(layer) => {
                    ev.main_window().set_layer(layer);
                }
                LayershellCustomActionsWithInfo::MarginChange(margin) => {
                    ev.main_window().set_margin(margin);
                }
                LayershellCustomActionsWithInfo::SizeChange((width, height)) => {
                    ev.main_window().set_size((width, height));
                }
                LayershellCustomActionsWithInfo::VirtualKeyboardPressed { time, key } => {
                    use layershellev::reexport::wayland_client::KeyState;
                    let ky = ev.get_virtual_keyboard().unwrap();
                    ky.key(time, key, KeyState::Pressed.into());
                    let eh = ev.get_loop_handler().unwrap();
                    let _ = eh.insert_source(
                        Timer::from_duration(Duration::from_micros(100)),
                        move |_, _, state| {
                            let ky = state.get_virtual_keyboard().unwrap();
                            ky.key(time, key, KeyState::Released.into());
                            TimeoutAction::Drop
                        },
                    )
                    .ok();
                }
                _ => {}
            },
            LayerShellActions::Mouse(mouse) => {
                let Some(pointer) = ev.get_pointer() else {
                    return ReturnData::None;
                };
                ev.append_return_data(ReturnData::RequestSetCursorShape((
                    conversion::mouse_interaction(mouse),
                    pointer.clone(),
                    pointer_serial,
                )));
            }
            LayerShellActions::RedrawAll => {
                ev.append_return_data(ReturnData::RedrawAllRequest);
            }
            LayerShellActions::RedrawWindow(index) => {
                ev.append_return_data(ReturnData::RedrawIndexRequest(index));
            }
            _ => {}
        }
        def_returndata
    });
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn run_instance<P, E, C>(
    program: P,
    mut program_state: P::State,
    compositor_settings: compositor::Settings,
    renderer_settings: renderer::Settings,
    mut runtime: SingleRuntime<E, P::Message>,
    mut event_receiver: mpsc::UnboundedReceiver<LayershellEvent<Action<P::Message>, ()>>,
    mut control_sender: mpsc::UnboundedSender<LayerShellActions<()>>,
    mut state: State<P>,
    window: Arc<WindowWrapper>,
    fonts: Vec<std::borrow::Cow<'static, [u8]>>,
) where
    P: Program + 'static,
    P::Theme: Default,
    E: Executor + 'static,
    C: Compositor<Renderer = P::Renderer> + 'static,
    P::Message: 'static + TryInto<LayershellCustomActions, Error = P::Message>,
{
    use enative_core::mouse;

    let shell = Shell::headless();
    let mut compositor = C::new(compositor_settings, window.clone(), window.clone(), shell)
        .await
        .expect("Cannot create compositor");

    for font in fonts {
        let _ = compositor.load_font(font);
    }

    let mut renderer = compositor.create_renderer(renderer_settings);

    let physical_size = state.physical_size();
    let cache = user_interface::Cache::default();
    let mut surface =
        compositor.create_surface(window.clone(), physical_size.width, physical_size.height);

    let mut should_exit = false;
    let mut clipboard = LayerShellClipboard::connect(&window);
    let mut mouse_interaction = mouse::Interaction::default();
    let mut messages = Vec::new();
    let mut events: Vec<NativeEvent> = Vec::new();
    let mut custom_actions = Vec::new();

    let window_id = state.window_id();

    let mut user_interface = ManuallyDrop::new(build_user_interface(
        &program,
        &program_state,
        window_id,
        cache,
        &mut renderer,
        state.logical_size(),
    ));

    while let Some(event) = event_receiver.next().await {
        match event {
            LayershellEvent::RequestRefresh { width, height } => {
                state.update_view_port(width, height);
                let ps = state.physical_size();
                let width = ps.width;
                let height = ps.height;

                user_interface =
                    ManuallyDrop::new(ManuallyDrop::into_inner(user_interface).relayout(
                        Size {
                            width: width as f32,
                            height: height as f32,
                        },
                        &mut renderer,
                    ));

                compositor.configure_surface(&mut surface, width, height);
                let redraw_event = NativeEvent::Window(
                    enative_core::window::Event::RedrawRequested(Instant::now()),
                );

                let (ui_state, _) = user_interface.update(
                    &[redraw_event.clone()],
                    state.cursor(),
                    &mut renderer,
                    &mut messages,
                );

                runtime.broadcast(subscription::Event::Interaction {
                    window: window_id,
                    event: redraw_event,
                    status: enative_core::event::Status::Ignored,
                });

                user_interface.draw(
                    &mut renderer,
                    state.theme(),
                    &renderer::Style {
                        text_color: state.text_color(),
                    },
                    state.cursor(),
                );

                let new_mouse_interaction = match &ui_state {
                    user_interface::State::Updated { mouse_interaction, .. } => *mouse_interaction,
                    user_interface::State::Outdated => mouse::Interaction::default(),
                };

                if new_mouse_interaction != mouse_interaction {
                    custom_actions.push(LayerShellActions::Mouse(new_mouse_interaction));
                    mouse_interaction = new_mouse_interaction;
                }

                let _ = compositor.present(
                    &mut renderer,
                    &mut surface,
                    state.viewport(),
                    state.background_color(),
                    || {},
                );
            }
            LayershellEvent::Window(window_event) => {
                state.update(&window_event);
                if let Some(native_event) =
                    conversion::window_event(window_id, &window_event, state.modifiers())
                {
                    events.push(native_event);
                }
            }
            LayershellEvent::NormalUpdate => {
                if events.is_empty() && messages.is_empty() {
                    continue;
                }

                let _ = user_interface.update(
                    &events,
                    state.cursor(),
                    &mut renderer,
                    &mut messages,
                );

                events.clear();

                if !messages.is_empty() {
                    let cache =
                        ManuallyDrop::into_inner(user_interface).into_cache();

                    update(
                        &program,
                        &mut program_state,
                        &mut state,
                        &mut runtime,
                        &mut messages,
                    );

                    user_interface = ManuallyDrop::new(build_user_interface(
                        &program,
                        &program_state,
                        window_id,
                        cache,
                        &mut renderer,
                        state.logical_size(),
                    ));
                }

                for action in custom_actions.drain(..) {
                    let _ = control_sender.start_send(action).ok();
                }
            }
            LayershellEvent::UserEvent(event) => {
                match event {
                    Action::Output(message) => {
                        messages.push(message);
                    }
                    Action::Exit => {
                        should_exit = true;
                    }
                    action => {
                        run_action(
                            &mut runtime,
                            action,
                            &mut messages,
                            &mut clipboard,
                            &mut should_exit,
                        );
                    }
                }
                if should_exit {
                    break;
                }
            }
            _ => {}
        }
    }

    let _ = ManuallyDrop::into_inner(user_interface);
}

fn build_user_interface<'a, P: Program>(
    program: &'a P,
    program_state: &'a P::State,
    window_id: window::Id,
    cache: user_interface::Cache,
    renderer: &mut P::Renderer,
    size: Size<f32>,
) -> UserInterface<'a, P::Message, P::Theme, P::Renderer> {
    let view = program.view(program_state, window_id);
    UserInterface::build(view, size, cache, renderer)
}

fn update<P: Program, E: Executor>(
    program: &P,
    program_state: &mut P::State,
    state: &mut State<P>,
    runtime: &mut SingleRuntime<E, P::Message>,
    messages: &mut Vec<P::Message>,
) where
    P::Theme: Default,
{
    for message in messages.drain(..) {
        let task = runtime.enter(|| program.update(program_state, message));
        if let Some(stream) = runtime_task::into_stream(task) {
            runtime.run(stream);
        }
    }
    state.synchronize(program, program_state);
    let sub = runtime.enter(|| program.subscription(program_state));
    runtime.track(subscription::into_recipes(sub.map(Action::Output)));
}

fn run_action<E: Executor, Message: Send + 'static>(
    _runtime: &mut Runtime<E, LayershellProxy<Action<Message>>, Action<Message>>,
    action: Action<Message>,
    messages: &mut Vec<Message>,
    clipboard: &mut LayerShellClipboard,
    should_exit: &mut bool,
) {
    use enative_runtime::clipboard;

    match action {
        Action::Output(message) => {
            messages.push(message);
        }
        Action::Clipboard(clipboard_action) => match clipboard_action {
            clipboard::Action::Read { kind, channel } => {
                clipboard.read(kind, move |result| {
                    let _ = channel.send(result);
                });
            }
            clipboard::Action::Write { content, channel } => {
                clipboard.write(content, move |result| {
                    let _ = channel.send(result);
                });
            }
        },
        Action::Exit => {
            *should_exit = true;
        }
        _ => {}
    }
}
