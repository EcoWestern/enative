use std::{borrow::Cow, fs::File};

use enative_core::{Font, Pixels};
use crate::reexport::{Anchor, KeyboardInteractivity, Layer};
use layershellev::reexport::wayland_client::wl_keyboard::KeymapFormat;

#[derive(Debug)]
pub struct VirtualKeyboardSettings {
    pub file: File,
    pub keymap_size: u32,
    pub keymap_format: KeymapFormat,
}

#[derive(Debug)]
pub struct Settings {
    pub id: Option<String>,
    pub layer_settings: LayerShellSettings,
    pub fonts: Vec<Cow<'static, [u8]>>,
    pub default_font: Font,
    pub default_text_size: Pixels,
    pub antialiasing: bool,
    pub virtual_keyboard_support: Option<VirtualKeyboardSettings>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            id: None,
            fonts: Vec::new(),
            layer_settings: LayerShellSettings::default(),
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            antialiasing: false,
            virtual_keyboard_support: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayerShellSettings {
    pub anchor: Anchor,
    pub layer: Layer,
    pub exclusive_zone: i32,
    pub size: Option<(u32, u32)>,
    pub margin: (i32, i32, i32, i32),
    pub keyboard_interactivity: KeyboardInteractivity,
    pub binded_output_name: Option<String>,
}

impl Default for LayerShellSettings {
    fn default() -> Self {
        LayerShellSettings {
            anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
            layer: Layer::Top,
            exclusive_zone: -1,
            size: None,
            margin: (0, 0, 0, 0),
            keyboard_interactivity: KeyboardInteractivity::OnDemand,
            binded_output_name: None,
        }
    }
}
