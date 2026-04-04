//! Layer shell integration for the eNative GUI engine.
//!
//! This crate provides support for running eNative [`Program`]s as
//! Wayland layer shell surfaces (docks, bars, overlays, etc.).
//!
//! [`Program`]: enative_program::Program
#![allow(missing_docs)]

pub mod actions;
pub mod application;
mod clipboard;
mod conversion;
mod error;
mod event;
mod proxy;
pub mod settings;

pub use error::Error;

pub mod reexport {
    //! Re-exports from `layershellev` for convenience.
    pub use layershellev::reexport::wayland_client::wl_keyboard;
    pub use layershellev::reexport::Anchor;
    pub use layershellev::reexport::KeyboardInteractivity;
    pub use layershellev::reexport::Layer;
    pub use layershellev::NewLayerShellSettings;
}

use crate::actions::LayershellCustomActions;
use enative_graphics::compositor;
use enative_program::Program;

/// Runs a [`Program`] as a Wayland layer shell surface.
pub fn run<P, C>(program: P, settings: settings::Settings) -> Result<(), Error>
where
    P: Program + 'static,
    P::Theme: Default,
    C: compositor::Compositor<Renderer = P::Renderer> + 'static,
    P::Message: 'static + TryInto<LayershellCustomActions, Error = P::Message>,
{
    application::run::<P, P::Executor, C>(program, settings)
}
