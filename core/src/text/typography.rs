//! Define the typography scale of the engine.
use crate::Pixels;

/// A typography preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Preset {
    /// The largest heading size. Usually 40px.
    H1,
    /// A large heading size. Usually 32px.
    H2,
    /// A medium heading size. Usually 24px.
    H3,
    /// The default body text size. Usually 16px.
    Body,
    /// A small label size. Usually 14px.
    Label,
    /// The smallest caption size. Usually 12px.
    Caption,
}

impl From<Preset> for Pixels {
    fn from(preset: Preset) -> Self {
        match preset {
            Preset::H1 => Pixels(40.0),
            Preset::H2 => Pixels(32.0),
            Preset::H3 => Pixels(24.0),
            Preset::Body => Pixels(16.0),
            Preset::Label => Pixels(14.0),
            Preset::Caption => Pixels(12.0),
        }
    }
}
