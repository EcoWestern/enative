<div align="center">

<img src="docs/logo.svg" width="160px" />

# eNative
### The UI Engine for the EcoWestern Ecosystem

[**Examples**](https://github.com/ecowestern/enative/tree/master/examples#examples)

---

eNative is a high-performance, cross-platform GUI library for Rust, designed for the rapid creation of native applications within the **EcoWestern Ecosystem**. 

Originally a fork of the [Iced] project, eNative is being tailored to provide a seamless development experience for **MatePC**, **MateOS**, **Windows**, **macOS**, **iOS**, and **Android**.

---

<a href="https://github.com/ecowestern/enative">
  <img src="https://enative.rs/showcase/halloy.gif" width="460px" style="border-radius: 8px; margin-bottom: 10px;">
</a>

</div>

## 🚀 Purpose & Vision

eNative serves as the foundational UI engine for **EcoWestern**. Our goal is to empower developers to build stunning, type-safe, and reactive interfaces that run anywhere—from the desktop power of MatePC to the mobile versatility of Android and iOS.

> [!IMPORTANT]
> **Status: Early Alpha**
> eNative currently inherits most existing bugs and quirks from [Iced]. We are actively working on resolving these issues and optimizing the engine for its new home in the EcoWestern family. **Expect rapid changes and improvements.**

## ✨ Features

* **Universal Reach**: Build once, deploy to **MatePC, MateOS, Windows, macOS, iOS, and Android**.
* **Type-Safe Reactive Model**: Inspired by [The Elm Architecture], ensuring your UI state is predictable and easy to debug.
* **Batteries-Included**: A rich set of built-in widgets (text inputs, scrollables, sliders, etc.) and first-class async support.
* **Modern Rendering**: Leveraging [`wgpu`] and [`tiny-skia`] for high-performance graphics with software fallbacks.
* **Responsive Layout**: Flexbox-like layout system that adapts to any screen size.
* **Debug Tooling**: Integrated performance metrics and time-traveling debugging (inherited from Iced).

## 🧩 Architectural Overview

eNative follows a simple and powerful pattern that splits your application into four core concepts:

1. **State**: The data driving your application.
2. **Messages**: Meaningful events (like button clicks) that trigger changes.
3. **View Logic**: Describing your UI as a tree of widgets based on the current state.
4. **Update Logic**: How your state should change in response to messages.

### A Minimal Example

Building a counter is as simple as:

```rust
use enative::widget::{button, column, text, Column};

#[derive(Default)]
struct Counter { value: i32 }

#[derive(Debug, Clone, Copy)]
pub enum Message { Increment, Decrement }

impl Counter {
    pub fn view(&self) -> Column<'_, Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
            button("-").on_press(Message::Decrement),
        ].into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }
}

fn main() -> enative::Result {
    enative::run(Counter::update, Counter::view)
}
```

## 🛠 Project Roadmap

As a fork of [Iced], our immediate focus is:
1. **Stabilization**: Fixing inherited bugs and edge cases.
2. **Platform Parity**: Deepening support for **MateOS** and **MatePC** unique features.
3. **Performance**: Optimizing the renderer for mobile and resource-constrained environments.
4. **EcoWestern Integration**: Direct bindings for EcoWestern services and ID systems.

## 🤝 Contributing & Community

We welcome contributors! Whether you're fixing a bug, suggesting a feature, or improving documentation, your help is appreciated.

* Read our [Contributing Guidelines](CONTRIBUTING.md).
* Report issues on our [GitHub repository](https://github.com/ecowestern/enative/issues).

---

<div align="center">
  Built with ❤️ by the EcoWestern Team
</div>

[Iced]: https://github.com/iced-rs/iced
[The Elm Architecture]: https://guide.elm-lang.org/architecture/
[`wgpu`]: https://github.com/gfx-rs/wgpu
[`tiny-skia`]: https://github.com/RazrFalcon/tiny-skia
