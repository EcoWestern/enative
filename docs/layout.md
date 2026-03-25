# Layout

eNative does not have a single unified layout engine. Instead, each widget implements its own layout strategy. In practice you compose a handful of core widgets — `column`, `row`, `container`, and `scrollable` — to achieve virtually any layout.

---

## The Core Primitives

### `column`

Stacks children **vertically** (top → bottom).

```rust
use enative::widget::column;

column![
    header(),
    body(),
    footer(),
]
.spacing(12)   // gap between items
.padding(16)   // space around the whole column
```

### `row`

Places children **horizontally** (left → right).

```rust
use enative::widget::row;

row![
    sidebar(),
    content(),
]
.spacing(8)
.align_y(enative::Center)   // vertically centre children
```

### `container`

Wraps a **single** child and controls where it sits within its own bounds.

```rust
use enative::widget::container;
use enative::Fill;

container(my_widget)
    .padding(20)
    .center_x(Fill)   // centre horizontally, take all available width
    .center_y(Fill)   // centre vertically,   take all available height
```

`container` is also commonly used to apply a background colour or a border to a widget:

```rust
container(content)
    .style(container::rounded_box)
```

---

## Sizing

The width and height of a widget can be controlled with a `Length` value. You set it by calling `.width(…)` and `.height(…)` on most widgets.

| Variant | Meaning |
|---|---|
| `enative::Fill` | Take all remaining space in the parent axis |
| `enative::Shrink` | Use only as much space as the content needs (default) |
| `enative::Fixed(px)` | Use exactly this many pixels |
| `enative::FillPortion(n)` | Take a proportional fraction of the available space |

```rust
use enative::{Fill, Shrink};
use enative::widget::{button, column, container, text};

// A full-width button
button("Full width").width(Fill)

// A container that fills the window
container(text("Centred!"))
    .width(Fill)
    .height(Fill)
    .center_x(Fill)
    .center_y(Fill)
```

### `FillPortion` — proportional sizing

Use `FillPortion` to divide space between siblings:

```rust
use enative::{FillPortion};
use enative::widget::row;

row![
    // sidebar takes 1/4 of the width
    sidebar.width(FillPortion(1)),
    // main content takes 3/4
    content.width(FillPortion(3)),
]
```

### Fixed pixel sizes

```rust
use enative::widget::container;

container(content).width(320).height(240)
```

Numeric literals are automatically converted to `Length::Fixed`.

---

## Spacing and Padding

- **`spacing`** — the gap *between* children of a `row` or `column`.
- **`padding`** — the space *inside* a widget, between its border and its contents.

```rust
use enative::widget::column;
use enative::Padding;

// Uniform padding
column![/* … */].padding(16)

// Asymmetric padding: (top, right, bottom, left)
column![/* … */].padding(Padding { top: 8.0, right: 16.0, bottom: 8.0, left: 16.0 })
```

---

## Alignment

Use `.align_x` and `.align_y` to align children inside a container or along the cross-axis of a row/column.

Common alignment constants:

| Constant | Meaning |
|---|---|
| `enative::Left` | Align to the left |
| `enative::Center` | Centre |
| `enative::Right` | Align to the right |
| `enative::Top` | Align to the top |
| `enative::Bottom` | Align to the bottom |

```rust
use enative::{Center, Right};
use enative::widget::column;

// Centre every child horizontally
column![/* … */].align_x(Center)

// Align children to the right edge
column![/* … */].align_x(Right)
```

---

## Scrollable Content

Wrap a column or row in a `scrollable` to handle overflow:

```rust
use enative::widget::{column, scrollable};
use enative::Fill;

scrollable(
    column(items.iter().map(item_view).collect::<Vec<_>>())
        .spacing(4)
)
.height(Fill)
```

By default `scrollable` scrolls vertically. Pass `scrollable::Direction::Horizontal` (or `Both`) to change this.

---

## Overlapping Widgets — `stack`

`stack` places widgets on top of each other. The last widget in the list is rendered on top.

```rust
use enative::widget::stack;

stack![
    background_image,
    overlay_text,
]
```

---

## Responsive Layouts

`responsive` lets you inspect the available width/height and return a different widget tree based on that:

```rust
use enative::widget::responsive;

responsive(|size| {
    if size.width > 900.0 {
        wide_layout()
    } else {
        narrow_layout()
    }
})
```

This is the recommended way to build adaptive layouts without needing breakpoints.

---

## A Full-Page Layout Example

```rust
use enative::widget::{column, container, row, scrollable, text};
use enative::{Fill, Element};

fn view(state: &State) -> Element<'_, Message> {
    let header = container(text("My App").size(28))
        .padding(16)
        .width(Fill)
        .style(container::dark);

    let sidebar = container(
        column(state.nav_items.iter().map(nav_item_view).collect())
            .spacing(4)
    )
    .width(200)
    .height(Fill)
    .padding(8);

    let content = scrollable(
        column(state.posts.iter().map(post_view).collect())
            .spacing(12)
            .padding(16),
    )
    .height(Fill)
    .width(Fill);

    let body = row![sidebar, content].height(Fill);

    column![header, body].into()
}
```

---

## Further Reading

- [Widgets](widgets.md) — all layout and display widgets
- [Theming & Styling](theming.md) — visual appearance of containers and widgets
