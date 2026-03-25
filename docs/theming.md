# Theming & Styling

eNative has a two-layer visual system:

1. **Theme** — a global palette that drives all default widget colours.
2. **Style** — per-widget overrides that let individual widgets look different from the theme default.

---

## Built-in Themes

eNative ships with a set of ready-to-use themes. Use `enative::Theme` to pick one:

| Variant | Description |
|---|---|
| `Theme::Light` | Clean light theme (default) |
| `Theme::Dark` | Classic dark theme |
| `Theme::Dracula` | Dracula colour scheme |
| `Theme::Nord` | Nord colour scheme |
| `Theme::SolarizedLight` / `Theme::SolarizedDark` | Solarized |
| `Theme::GruvboxLight` / `Theme::GruvboxDark` | Gruvbox |
| `Theme::CatppuccinLatte` / `…Frappe` / `…Macchiato` / `…Mocha` | Catppuccin |
| `Theme::TokyoNight` / `Theme::TokyoNightStorm` / `Theme::TokyoNightLight` | Tokyo Night |
| `Theme::KanagawaWave` / `Theme::KanagawaDragon` / `Theme::KanagawaLotus` | Kanagawa |
| `Theme::Moonfly` / `Theme::Nightfly` | Dark terminal-inspired themes |
| `Theme::Oxocarbon` | IBM Oxocarbon |
| `Theme::Ferra` | Warm dark theme |

### Applying a theme

Use `enative::application` and provide a `theme` function. The function receives the current state so the theme can be dynamic (e.g. user-controlled dark mode):

```rust
use enative::Theme;

pub fn main() -> enative::Result {
    enative::application(new, update, view)
        .theme(theme)
        .run()
}

fn theme(state: &MyState) -> Theme {
    if state.dark_mode {
        Theme::Dark
    } else {
        Theme::Light
    }
}
```

---

## Custom Themes

Call `Theme::custom` (or `Theme::custom_with_fn`) to define your own palette:

```rust
use enative::theme::{Custom, Palette};
use enative::{Color, Theme};

fn my_theme() -> Theme {
    Theme::Custom(Box::new(Custom::new(
        "BrandTheme",
        Palette {
            background: Color::from_rgb(0.98, 0.97, 0.95),
            text: Color::from_rgb(0.1, 0.1, 0.1),
            primary: Color::from_rgb(0.2, 0.5, 0.9),
            success: Color::from_rgb(0.2, 0.75, 0.4),
            danger: Color::from_rgb(0.9, 0.2, 0.2),
        },
    )))
}
```

The `Palette` drives the default colours for every widget in the app.

---

## The Palette

You can read the current theme's palette anywhere in your `view` or style closure:

```rust
let palette = theme.palette();

// palette.primary   — main accent colour
// palette.success   — green / positive
// palette.danger    — red / destructive
// palette.text      — default text colour
// palette.background — window background
```

Each colour also exposes `.base`, `.weak`, and `.strong` variants through `theme.extended_palette()` for fine-grained control.

---

## Styling Individual Widgets

Every widget that has visual states exposes a `.style(…)` method. You pass a closure that receives the active `Theme` (and sometimes a `Status`) and returns the widget's `Style` struct.

### Using a preset

Most widget modules export convenient styling presets:

```rust
use enative::widget::{button, container, text};

// container presets
container(content).style(container::rounded_box)
container(content).style(container::dark)

// button presets
button("Primary").style(button::primary)
button("Danger").style(button::danger)
button("Success").style(button::success)
button("Secondary").style(button::secondary)
button("Text-only").style(button::text)

// text presets
text("Warning").style(text::danger)
text("Success!").style(text::success)
```

### Writing a custom closure

```rust
use enative::widget::button;
use enative::{Border, Color, Theme};

button("Custom").style(|theme: &Theme, status| {
    let palette = theme.palette();

    let background = match status {
        button::Status::Active => palette.primary,
        button::Status::Hovered => Color {
            a: 0.85,
            ..palette.primary
        },
        button::Status::Pressed => Color {
            a: 0.7,
            ..palette.primary
        },
        button::Status::Disabled => Color {
            a: 0.4,
            ..palette.primary
        },
    };

    button::Style {
        background: Some(background.into()),
        border: Border::rounded(8),
        text_color: Color::WHITE,
        ..Default::default()
    }
})
```

The `Status` enum is specific to each widget type. Check the widget's module docs for the exact variants.

---

## Colours

eNative uses `enative::Color` — an RGBA float colour.

```rust
use enative::Color;

// From floats (0.0–1.0 range)
let red = Color::from_rgb(1.0, 0.0, 0.0);

// From u8 values (0–255 range)
let green = Color::from_rgb8(0, 200, 100);

// From hex using the `color!` macro
let blue = enative::color!(0x3498DB);          // RGB
let semi = enative::color!(0x3498DB, 0.5);     // RGB + alpha

// Predefined constants
let white = Color::WHITE;
let black = Color::BLACK;
let transparent = Color::TRANSPARENT;
```

---

## Fonts

eNative uses **Fira Sans** as its default font (embedded when the `fira-sans` feature is enabled) and falls back to the operating system's default sans-serif.

### Loading a custom font

```rust
use enative::Font;

// Pass font bytes at startup via the application builder
enative::application(new, update, view)
    .font(include_bytes!("../fonts/MyFont-Regular.ttf").as_slice())
    .run()
```

### Applying a font to text

```rust
use enative::widget::text;
use enative::Font;

text("Custom font").font(Font::with_name("MyFont"))
text("Bold").font(Font { weight: enative::font::Weight::Bold, ..Font::default() })
```

---

## Text Size and Line Height

```rust
use enative::widget::text;
use enative::Pixels;

text("Large").size(32)
text("Small").size(Pixels(11.0)).line_height(1.4)
```

---

## Background and Borders

`Background` and `Border` are used inside style structs.

```rust
use enative::{Background, Border, Color};
use enative::gradient::Linear;

// Solid colour background
Background::Color(Color::from_rgb8(30, 30, 46))

// Linear gradient background
Background::Gradient(
    Linear::new(0.0)   // angle in radians
        .add_stop(0.0, Color::from_rgb8(26, 27, 38))
        .add_stop(1.0, Color::from_rgb8(36, 40, 59))
        .into()
)

// Rounded border
Border {
    color: Color::from_rgb8(68, 71, 90),
    width: 1.0,
    radius: 8.0.into(),
}
```

---

## Shadows

```rust
use enative::Shadow;

Shadow {
    color: Color::BLACK,
    offset: enative::Vector::new(0.0, 4.0),
    blur_radius: 12.0,
}
```

---

## Further Reading

- [Widgets](widgets.md) — list of all widgets and their modules
- [Layout](layout.md) — sizing and positioning
- [`examples/styling`](../examples/styling) — runnable light/dark mode demo
