//! The official renderer for enative.
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "wgpu-bare")]
pub use enative_wgpu as wgpu;

pub mod fallback;

pub use enative_graphics as graphics;
pub use enative_graphics::core;

#[cfg(feature = "geometry")]
pub use enative_graphics::geometry;

/// The default graphics renderer for [`enative`].
///
/// [`enative`]: https://github.com/enative-rs/enative
pub type Renderer = renderer::Renderer;

/// The default graphics compositor for [`enative`].
///
/// [`enative`]: https://github.com/enative-rs/enative
pub type Compositor = renderer::Compositor;

#[cfg(all(feature = "wgpu-bare", feature = "tiny-skia"))]
mod renderer {
    pub type Renderer =
        crate::fallback::Renderer<enative_wgpu::Renderer, enative_tiny_skia::Renderer>;

    pub type Compositor = crate::fallback::Compositor<
        enative_wgpu::window::Compositor,
        enative_tiny_skia::window::Compositor,
    >;
}

#[cfg(all(feature = "wgpu-bare", not(feature = "tiny-skia")))]
mod renderer {
    pub type Renderer = enative_wgpu::Renderer;
    pub type Compositor = enative_wgpu::window::Compositor;
}

#[cfg(all(not(feature = "wgpu-bare"), feature = "tiny-skia"))]
mod renderer {
    pub type Renderer = enative_tiny_skia::Renderer;
    pub type Compositor = enative_tiny_skia::window::Compositor;
}

#[cfg(not(any(feature = "wgpu-bare", feature = "tiny-skia")))]
mod renderer {
    #[cfg(not(debug_assertions))]
    compile_error!(
        "Cannot compile `enative_renderer` in release mode \
        without a renderer feature enabled. \
        Enable either the `wgpu` or `tiny-skia` feature, or both."
    );

    pub type Renderer = ();
    pub type Compositor = ();
}
