# Widgets

eNative ships with more than 30 built-in widgets. They are all available in the `enative::widget` module.

Widgets are **immutable descriptions** of UI — they do not hold state. The `view` function re-creates the widget tree every time the state changes, and eNative only redraws the parts that actually differ.

---

## Using Widgets

Most widgets are created through free functions or macros:

```rust
use enative::widget::{button, column, row, text, text_input};
use enative::Element;

fn view(state: &MyState) -> Element<'_, Message> {
    column![
        text("Hello, World!").size(24),
        row![
            button("Cancel").on_press(Message::Cancel),
            button("OK").on_press(Message::Confirm),
        ]
        .spacing(8),
    ]
    .spacing(16)
    .padding(20)
    .into()
}
```

All widgets use a **builder pattern**: call methods to configure them and then `.into()` to convert them to an `Element`.

---

## Widget Reference

### Layout Widgets

#### `column!` / `Column`

Arranges children **vertically**, top to bottom.

```rust
use enative::widget::column;

column![child_a, child_b, child_c]
    .spacing(10)   // pixels between children
    .padding(20)   // pixels around all children
    .align_x(enative::Center)
```

#### `row!` / `Row`

Arranges children **horizontally**, left to right.

```rust
use enative::widget::row;

row![left_widget, right_widget]
    .spacing(8)
    .align_y(enative::Center)
```

#### `container`

Wraps a **single** widget and lets you control its alignment and padding.

```rust
use enative::widget::container;
use enative::Fill;

container(my_widget)
    .padding(16)
    .center_x(Fill)
    .center_y(Fill)
```

#### `stack`

Overlays widgets on top of each other (like CSS `position: absolute`).

```rust
use enative::widget::stack;

stack![background_image, foreground_text]
```

#### `scrollable`

Makes content scrollable when it overflows its bounds.

```rust
use enative::widget::scrollable;

scrollable(column![/* long list */])
    .height(300)
```

#### `pane_grid`

A resizable, draggable panel layout — useful for IDE-style interfaces.  
See the [`pane_grid` example](../examples/pane_grid) for a full walkthrough.

#### `grid`

A fixed-column grid layout.

```rust
use enative::widget::grid;

grid(vec![item_a, item_b, item_c, item_d])
    .column_width(enative::Fixed(120.0))
```

#### `table`

A table with sortable, resizable columns.  
See the [`table` example](../examples/table).

---

### Input Widgets

#### `button`

A pressable button. It only emits a message when `.on_press(Message::Something)` is set; without `on_press` the button is disabled.

```rust
use enative::widget::button;

button("Click me").on_press(Message::Clicked)
button(icon_widget).on_press(Message::IconAction)   // any widget as content
```

#### `text_input`

A single-line text field.

```rust
use enative::widget::text_input;

text_input("Placeholder…", &state.value)
    .on_input(Message::ValueChanged)   // called on every keystroke
    .on_submit(Message::Submitted)     // called when Enter is pressed
    .password()                        // masks input
    .id(text_input::Id::new("my-field"))
```

#### `text_editor`

A multi-line text editor with rich content support, undo/redo, and optional syntax highlighting.

```rust
use enative::widget::text_editor;

text_editor(&state.content)
    .on_action(Message::EditorAction)
    .height(enative::Fill)
```

#### `checkbox`

```rust
use enative::widget::checkbox;

checkbox("Accept terms", state.accepted)
    .on_toggle(Message::ToggleAccepted)
```

#### `radio`

Radio buttons are typically rendered as a group inside a `column`:

```rust
use enative::widget::radio;

column![
    radio("Option A", MyOption::A, Some(state.selected), Message::OptionSelected),
    radio("Option B", MyOption::B, Some(state.selected), Message::OptionSelected),
    radio("Option C", MyOption::C, Some(state.selected), Message::OptionSelected),
]
```

#### `toggler`

A toggle switch (on/off).

```rust
use enative::widget::toggler;

toggler(state.dark_mode)
    .label("Dark mode")
    .on_toggle(Message::DarkModeToggled)
```

#### `pick_list`

A dropdown / select widget.

```rust
use enative::widget::pick_list;

pick_list(
    &["Small", "Medium", "Large"][..],
    state.selected_size.as_deref(),
    Message::SizeSelected,
)
```

#### `combo_box`

A searchable dropdown that allows both selecting from a list and typing a custom value.

```rust
use enative::widget::combo_box;

combo_box(&state.combo_state, "Search…", state.selected.as_deref(), Message::Selected)
```

#### `slider` / `vertical_slider`

```rust
use enative::widget::slider;

slider(0.0..=100.0, state.volume, Message::VolumeChanged)
    .step(1.0)
```

---

### Display Widgets

#### `text`

Renders a string or any value that implements `Display`.

```rust
use enative::widget::text;

text("Hello!").size(24).color(enative::color!(0xFF0000))
text(state.count).size(48)   // renders a number directly
```

#### `image`

Requires the `image` feature.

```rust
use enative::widget::image;

image("assets/logo.png").width(120)
```

#### `svg`

Requires the `svg` feature.

```rust
use enative::widget::svg;

svg("assets/icon.svg").height(48)
```

#### `canvas`

A 2-D drawing surface for custom graphics. Requires the `canvas` feature.  
See the [`clock`](../examples/clock), [`bezier_tool`](../examples/bezier_tool), and [`game_of_life`](../examples/game_of_life) examples.

```rust
use enative::widget::canvas;

canvas(my_program).width(enative::Fill).height(enative::Fill)
```

#### `markdown`

Renders a Markdown string. Requires the `markdown` feature.

```rust
use enative::widget::markdown;

markdown(&state.markdown_content, markdown::Settings::default(), markdown::Style::from_palette(theme.palette()))
    .map(Message::LinkClicked)
```

#### `qr_code`

Generates and renders a QR code. Requires the `qr_code` feature.

```rust
use enative::widget::qr_code;

qr_code(qr_code::Data::new("https://example.com")?)
```

---

### Interactive / Special Widgets

#### `tooltip`

Displays a floating label when the user hovers over a widget.

```rust
use enative::widget::tooltip;

tooltip(
    button("?").on_press(Message::Help),
    "Click for help",
    tooltip::Position::Bottom,
)
```

#### `mouse_area`

Attaches mouse event handlers to any widget.

```rust
use enative::widget::mouse_area;

mouse_area(my_widget)
    .on_press(Message::AreaClicked)
    .on_right_press(Message::ContextMenu)
```

#### `responsive`

Gives you the available size so you can adapt your layout dynamically.

```rust
use enative::widget::responsive;

responsive(|size| {
    if size.width > 800.0 {
        wide_layout()
    } else {
        narrow_layout()
    }
})
```

#### `lazy`

Only re-renders its content when a tracked value changes. Requires the `lazy` feature.

```rust
use enative::widget::lazy;

lazy(state.items.len(), |_| expensive_list_view(&state.items))
```

---

## Custom Widgets

If the built-in widgets don't cover your needs, you can build your own by implementing the `Widget` trait. See the [`custom_widget`](../examples/custom_widget) example for a minimal walkthrough.

---

## Further Reading

- [Layout](layout.md) — how to compose and size widgets
- [Theming & Styling](theming.md) — changing the look of widgets
