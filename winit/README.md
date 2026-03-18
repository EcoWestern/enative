# `enative_winit`
[![Documentation](https://docs.rs/enative_winit/badge.svg)][documentation]
[![Crates.io](https://img.shields.io/crates/v/enative_winit.svg)](https://crates.io/crates/enative_winit)
[![License](https://img.shields.io/crates/l/enative_winit.svg)](https://github.com/enative-rs/enative/blob/master/LICENSE)
[![Discord Server](https://img.shields.io/discord/628993209984614400?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8)](https://discord.gg/3xZJ65GAhd)

`enative_winit` offers some convenient abstractions on top of [`enative_native`] to quickstart development when using [`winit`].

It exposes a renderer-agnostic `Application` trait that can be implemented and then run with a simple call. The use of this trait is optional. A `conversion` module is provided for users that decide to implement a custom event loop.

<p align="center">
  <img alt="The native target" src="../docs/graphs/native.png" width="80%">
</p>

[documentation]: https://docs.rs/enative_winit
[`enative_native`]: ../native
[`winit`]: https://github.com/rust-windowing/winit
