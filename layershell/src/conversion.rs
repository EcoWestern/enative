mod keymap;

use crate::event::ButtonState2;
use crate::event::WindowEvent as LayerShellEvent;
use enative_core::touch;
use enative_core::SmolStr;
use enative_core::{keyboard, mouse, Event as NativeEvent};
use keymap::{key, physical_key};
use layershellev::keyboard::KeyLocation;
use layershellev::keyboard::ModifiersState;
use layershellev::xkb_keyboard::ElementState;
use layershellev::xkb_keyboard::KeyEvent as LayerShellKeyEvent;

pub fn window_event(
    #[allow(unused)] id: enative_core::window::Id,
    layerevent: &LayerShellEvent,
    modifiers: ModifiersState,
) -> Option<NativeEvent> {
    match layerevent {
        LayerShellEvent::CursorLeft => Some(NativeEvent::Mouse(mouse::Event::CursorLeft)),
        LayerShellEvent::CursorMoved { x, y } => {
            Some(NativeEvent::Mouse(mouse::Event::CursorMoved {
                position: enative_core::Point {
                    x: *x as f32,
                    y: *y as f32,
                },
            }))
        }
        LayerShellEvent::CursorEnter { .. } => {
            Some(NativeEvent::Mouse(mouse::Event::CursorEntered))
        }
        LayerShellEvent::MouseInput(state) => Some(NativeEvent::Mouse(match state {
            ButtonState2::Pressed(btn) => mouse::Event::ButtonPressed(*btn),
            ButtonState2::Released(btn) => mouse::Event::ButtonReleased(*btn),
        })),
        LayerShellEvent::Axis { x, y } => Some(NativeEvent::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Lines { x: *x, y: *y },
        })),
        LayerShellEvent::PixelDelta { x, y } => {
            Some(NativeEvent::Mouse(mouse::Event::WheelScrolled {
                delta: mouse::ScrollDelta::Pixels { x: *x, y: *y },
            }))
        }
        LayerShellEvent::KeyBoardInput { event, .. } => Some(NativeEvent::Keyboard({
            let key = event.key_without_modifiers();
            let text = event
                .text_with_all_modifiers()
                .map(SmolStr::new)
                .filter(|text| !text.as_str().chars().any(is_private_use));
            let LayerShellKeyEvent {
                state,
                location,
                logical_key,
                physical_key,
                ..
            } = event;
            let key = self::key(key);
            let modified_key = self::key(logical_key.clone());
            let physical_key = self::physical_key(*physical_key);
            let modifiers = keymap::modifiers(modifiers);
            let location = match location {
                KeyLocation::Standard => keyboard::Location::Standard,
                KeyLocation::Left => keyboard::Location::Left,
                KeyLocation::Right => keyboard::Location::Right,
                KeyLocation::Numpad => keyboard::Location::Numpad,
            };
            match state {
                ElementState::Pressed => keyboard::Event::KeyPressed {
                    key,
                    location,
                    modifiers,
                    text,
                    modified_key,
                    physical_key,
                    repeat: false,
                },
                ElementState::Released => keyboard::Event::KeyReleased {
                    key,
                    location,
                    modifiers,
                    modified_key,
                    physical_key,
                },
            }
        })),
        LayerShellEvent::TouchDown { id, x, y } => {
            Some(NativeEvent::Touch(touch::Event::FingerPressed {
                id: touch::Finger(*id as u64),
                position: enative_core::Point {
                    x: *x as f32,
                    y: *y as f32,
                },
            }))
        }
        LayerShellEvent::TouchUp { id, x, y } => {
            Some(NativeEvent::Touch(touch::Event::FingerLifted {
                id: touch::Finger(*id as u64),
                position: enative_core::Point {
                    x: *x as f32,
                    y: *y as f32,
                },
            }))
        }
        LayerShellEvent::TouchMotion { id, x, y } => {
            Some(NativeEvent::Touch(touch::Event::FingerMoved {
                id: touch::Finger(*id as u64),
                position: enative_core::Point {
                    x: *x as f32,
                    y: *y as f32,
                },
            }))
        }
        LayerShellEvent::TouchCancel { id, x, y } => {
            Some(NativeEvent::Touch(touch::Event::FingerLost {
                id: touch::Finger(*id as u64),
                position: enative_core::Point {
                    x: *x as f32,
                    y: *y as f32,
                },
            }))
        }
        LayerShellEvent::ModifiersChanged(new_modifiers) => Some(NativeEvent::Keyboard(
            keyboard::Event::ModifiersChanged(keymap::modifiers(*new_modifiers)),
        )),
        _ => None,
    }
}

pub(crate) fn mouse_interaction(interaction: mouse::Interaction) -> String {
    use mouse::Interaction;
    match interaction {
        Interaction::Idle => "default".to_owned(),
        Interaction::Pointer => "pointer".to_owned(),
        Interaction::Progress => "progress".to_owned(),
        Interaction::Grab => "grab".to_owned(),
        Interaction::Text => "text".to_owned(),
        Interaction::ZoomIn => "zoom_in".to_owned(),
        Interaction::Grabbing => "grabbing".to_owned(),
        Interaction::Crosshair => "crosshair".to_owned(),
        Interaction::NotAllowed => "not_allowed".to_owned(),
        Interaction::ResizingVertically => "ns_resize".to_owned(),
        Interaction::ResizingHorizontally => "ew_resize".to_owned(),
        _ => "default".to_owned(),
    }
}

fn is_private_use(c: char) -> bool {
    ('\u{E000}'..='\u{F8FF}').contains(&c)
}
