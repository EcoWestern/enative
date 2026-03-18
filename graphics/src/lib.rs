//! A bunch of backend-agnostic types that can be leveraged to build a renderer
//! for [`enative`].
//!
//! ![The native path of the enative ecosystem](https://github.com/enative-rs/enative/blob/0525d76ff94e828b7b21634fa94a747022001c83/docs/graphs/native.png?raw=true)
//!
//! [`enative`]: https://github.com/enative-rs/enative
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/enative-rs/enative/9ab6923e943f784985e9ef9ca28b10278297225d/docs/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
mod antialiasing;
mod viewport;

pub mod cache;
pub mod color;
pub mod compositor;
pub mod damage;
pub mod error;
pub mod gradient;
pub mod image;
pub mod layer;
pub mod mesh;
pub mod shell;
pub mod text;

#[cfg(feature = "geometry")]
pub mod geometry;

pub use antialiasing::Antialiasing;
pub use cache::Cache;
pub use compositor::Compositor;
pub use error::Error;
pub use gradient::Gradient;
pub use image::Image;
pub use layer::Layer;
pub use mesh::Mesh;
pub use shell::Shell;
pub use text::Text;
pub use viewport::Viewport;

pub use enative_core as core;
pub use enative_futures as futures;
