# `enative_wgpu`
[![Documentation](https://docs.rs/enative_wgpu/badge.svg)][documentation]
[![Crates.io](https://img.shields.io/crates/v/enative_wgpu.svg)](https://crates.io/crates/enative_wgpu)
[![License](https://img.shields.io/crates/l/enative_wgpu.svg)](https://github.com/enative-rs/enative/blob/master/LICENSE)
[![Discord Server](https://img.shields.io/discord/628993209984614400?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8)](https://discord.gg/3xZJ65GAhd)

`enative_wgpu` is a [`wgpu`] renderer for [`enative_runtime`]. For now, it is the default renderer of enative on [native platforms].

[`wgpu`] supports most modern graphics backends: Vulkan, Metal, DX12, OpenGL, and WebGPU.

<p align="center">
  <img alt="The native target" src="../docs/graphs/native.png" width="80%">
</p>

[documentation]: https://docs.rs/enative_wgpu
[`enative_runtime`]: ../runtime
[`wgpu`]: https://github.com/gfx-rs/wgpu
[native platforms]: https://github.com/gfx-rs/wgpu#supported-platforms
[WebGPU API]: https://gpuweb.github.io/gpuweb/
[`wgpu_glyph`]: https://github.com/hecrj/wgpu_glyph
