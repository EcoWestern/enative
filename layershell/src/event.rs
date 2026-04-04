use enative_core::mouse;
use layershellev::keyboard::ModifiersState;
use layershellev::reexport::wayland_client::{ButtonState, KeyState, WEnum};
use layershellev::xkb_keyboard::KeyEvent as LayerShellKeyEvent;
use layershellev::{DispatchMessage, WindowWrapper};

use enative_core::keyboard::Modifiers as IcedModifiers;

use crate::actions::LayershellNewMenuSettings;

fn from_u32_to_mouse(code: u32) -> mouse::Button {
    match code {
        273 => mouse::Button::Right,
        _ => mouse::Button::Left,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonState2 {
    Pressed(mouse::Button),
    Released(mouse::Button),
}

#[derive(Debug, Clone, Copy)]
pub enum KeyState2 {
    Pressed,
    Released,
}

impl From<WEnum<KeyState>> for KeyState2 {
    fn from(value: WEnum<KeyState>) -> Self {
        match value {
            WEnum::Value(KeyState::Released) => Self::Released,
            WEnum::Value(KeyState::Pressed) => Self::Pressed,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WindowEvent {
    ScaleChanged(u32),
    CursorEnter {
        x: f64,
        y: f64,
    },
    CursorMoved {
        x: f64,
        y: f64,
    },
    CursorLeft,
    MouseInput(ButtonState2),
    Keyboard {
        state: KeyState2,
        key: u32,
        modifiers: IcedModifiers,
    },
    KeyBoardInput {
        event: LayerShellKeyEvent,
        is_synthetic: bool,
    },
    ModifiersChanged(ModifiersState),
    Axis {
        x: f32,
        y: f32,
    },
    PixelDelta {
        x: f32,
        y: f32,
    },
    TouchDown {
        id: i32,
        x: f64,
        y: f64,
    },
    TouchUp {
        id: i32,
        x: f64,
        y: f64,
    },
    TouchMotion {
        id: i32,
        x: f64,
        y: f64,
    },
    TouchCancel {
        id: i32,
        x: f64,
        y: f64,
    },
}

#[derive(Debug)]
pub enum LayershellEvent<Message: 'static, INFO: Clone> {
    RequestRefreshWithWrapper {
        width: u32,
        height: u32,
        wrapper: WindowWrapper,
        is_created: bool,
        info: Option<INFO>,
    },
    RequestRefresh {
        width: u32,
        height: u32,
    },
    Window(WindowEvent),
    NormalUpdate,
    UserEvent(Message),
    WindowRemoved(enative_core::window::Id),
    NewMenu((LayershellNewMenuSettings, INFO)),
}

impl<Message: 'static, INFO: Clone> From<&DispatchMessage> for LayershellEvent<Message, INFO> {
    fn from(value: &DispatchMessage) -> Self {
        match value {
            DispatchMessage::RequestRefresh { width, height, .. } => {
                LayershellEvent::RequestRefresh {
                    width: *width,
                    height: *height,
                }
            }
            DispatchMessage::MouseEnter {
                surface_x: x,
                surface_y: y,
                ..
            } => LayershellEvent::Window(WindowEvent::CursorEnter { x: *x, y: *y }),
            DispatchMessage::MouseMotion {
                surface_x: x,
                surface_y: y,
                ..
            } => LayershellEvent::Window(WindowEvent::CursorMoved { x: *x, y: *y }),
            DispatchMessage::MouseLeave => LayershellEvent::Window(WindowEvent::CursorLeft),
            DispatchMessage::MouseButton { state, button, .. } => {
                let btn = from_u32_to_mouse(*button);
                match state {
                    WEnum::Value(ButtonState::Pressed) => {
                        LayershellEvent::Window(WindowEvent::MouseInput(ButtonState2::Pressed(btn)))
                    }
                    WEnum::Value(ButtonState::Released) => LayershellEvent::Window(
                        WindowEvent::MouseInput(ButtonState2::Released(btn)),
                    ),
                    _ => unreachable!(),
                }
            }
            DispatchMessage::TouchUp { id, x, y, .. } => {
                LayershellEvent::Window(WindowEvent::TouchUp {
                    id: *id,
                    x: *x,
                    y: *y,
                })
            }
            DispatchMessage::TouchDown { id, x, y, .. } => {
                LayershellEvent::Window(WindowEvent::TouchDown {
                    id: *id,
                    x: *x,
                    y: *y,
                })
            }
            DispatchMessage::TouchMotion { id, x, y, .. } => {
                LayershellEvent::Window(WindowEvent::TouchMotion {
                    id: *id,
                    x: *x,
                    y: *y,
                })
            }
            DispatchMessage::TouchCancel { id, x, y, .. } => {
                LayershellEvent::Window(WindowEvent::TouchCancel {
                    id: *id,
                    x: *x,
                    y: *y,
                })
            }
            DispatchMessage::PrefredScale(scale) => {
                LayershellEvent::Window(WindowEvent::ScaleChanged(*scale))
            }
            DispatchMessage::KeyboardInput {
                event,
                is_synthetic,
            } => LayershellEvent::Window(WindowEvent::KeyBoardInput {
                event: event.clone(),
                is_synthetic: *is_synthetic,
            }),
            DispatchMessage::ModifiersChanged(modifiers) => {
                LayershellEvent::Window(WindowEvent::ModifiersChanged(*modifiers))
            }
            DispatchMessage::Axis {
                horizontal,
                vertical,
                ..
            } => {
                if horizontal.stop && vertical.stop {
                    return Self::NormalUpdate;
                }
                let has_scroll = vertical.discrete != 0 || horizontal.discrete != 0;
                if has_scroll {
                    return LayershellEvent::Window(WindowEvent::Axis {
                        x: -horizontal.discrete as f32,
                        y: -vertical.discrete as f32,
                    });
                }
                LayershellEvent::Window(WindowEvent::Axis {
                    x: -horizontal.absolute as f32,
                    y: -vertical.absolute as f32,
                })
            }
        }
    }
}
