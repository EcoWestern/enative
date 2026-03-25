# Getting Started

This guide walks you through setting up a new eNative project from scratch and writing your first application.

## Prerequisites

### Rust

eNative requires **Rust 1.92 or later** (MSRV). If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify your installation:

```bash
rustc --version   # should print 1.92.0 or higher
cargo --version
```

### System Dependencies

On Linux you need a few extra packages. See [`DEPENDENCIES.md`](../DEPENDENCIES.md) for a full list.

**Debian / Ubuntu**

```bash
sudo apt-get install libxkbcommon-dev libvulkan-dev
```

**NixOS** — a `shell.nix` / `flake.nix` with everything pre-configured is provided in `DEPENDENCIES.md`.

macOS and Windows have no additional requirements.

---

## Creating a New Project

```bash
cargo new my_app
cd my_app
```

Add eNative to `Cargo.toml`:

```toml
[dependencies]
enative = "0.15"
```

> The default feature set enables the `wgpu` GPU renderer, the `tiny-skia` software fallback, and the thread-pool async executor. You usually do not need to change the defaults.

---

## Your First Application — A Counter

The classic "Hello, World" of eNative is a counter with increment and decrement buttons.

Replace `src/main.rs` with the following:

```rust
use enative::Center;
use enative::widget::{button, column, text};

pub fn main() -> enative::Result {
    enative::run(Counter::update, Counter::view)
}

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> enative::widget::Column<'_, Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement),
        ]
        .padding(20)
        .align_x(Center)
    }
}
```

Run it:

```bash
cargo run
```

You should see a window with two buttons and a number between them. Clicking **Increment** adds one; **Decrement** subtracts one.

---

## Project Layout

A typical eNative project has a very flat structure:

```
my_app/
├── Cargo.toml
└── src/
    └── main.rs   # (or lib.rs for a library)
```

Larger projects often split `main.rs` into modules by concern:

```
src/
├── main.rs          # entry point — calls enative::run / enative::application
├── state.rs         # application state struct
├── message.rs       # Message enum
├── update.rs        # update function
└── view/
    ├── mod.rs       # top-level view function
    ├── header.rs    # sub-views
    └── sidebar.rs
```

There is no enforced structure — use whatever organisation feels natural for your project.

---

## Useful Cargo Features

eNative exposes a number of opt-in features. Add them to `Cargo.toml` as needed:

| Feature | What it enables |
|---|---|
| `svg` | `widget::Svg` for rendering SVG files |
| `canvas` | `widget::Canvas` for custom 2-D drawing |
| `image` | `widget::Image` for raster images |
| `markdown` | `widget::Markdown` for rendering Markdown |
| `qr_code` | `widget::QRCode` |
| `debug` | Performance overlay (press **F12** at runtime) |
| `time-travel` | Time-travel debugging (experimental) |
| `hot` | Hot-reloading (experimental) |
| `tokio` | Use Tokio as the async executor instead of the built-in thread-pool |
| `smol` | Use Smol as the async executor |

Example:

```toml
[dependencies]
enative = { version = "0.15", features = ["svg", "canvas", "image", "debug"] }
```

---

## Running the Bundled Examples

The repository ships with more than 40 example applications under `examples/`. Each one can be run directly with Cargo:

```bash
# Clone the repository (or use your local copy)
git clone https://github.com/ecowestern/enative
cd enative

# Run any example by its package name
cargo run --package counter
cargo run --package todos
cargo run --package tour
cargo run --package game_of_life
```

See [`examples/README.md`](../examples/README.md) for the full list.

---

## Next Steps

| Topic | Document |
|---|---|
| Understand the core pattern | [Architecture](architecture.md) |
| All available widgets | [Widgets](widgets.md) |
| Arranging widgets on screen | [Layout](layout.md) |
| Colours, fonts, and dark mode | [Theming & Styling](theming.md) |
| Networking, timers, background work | [Async — Tasks & Subscriptions](async.md) |
