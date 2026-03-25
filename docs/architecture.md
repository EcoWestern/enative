# Architecture

eNative follows **The Elm Architecture** — a simple, unidirectional data-flow pattern that keeps your application state predictable and easy to reason about.

Every eNative application is made up of four things:

| Concept | What it is |
|---|---|
| **State** | A value (usually a `struct`) that holds all the data your UI needs |
| **Messages** | An `enum` whose variants describe every meaningful event that can happen |
| **Update** | A function that receives a `Message` and mutates the `State` |
| **View** | A function that turns the current `State` into a tree of widgets |

The runtime wires them together:

```
User interaction / async event
        │
        ▼
   Message produced
        │
        ▼
  update(&mut State, Message)
        │
        ▼
  view(&State) → Element
        │
        ▼
  Render to screen
```

This cycle repeats every time an event occurs. Because state can only change through `update`, it is always easy to trace *why* the UI looks the way it does.

---

## State

State is just a Rust value — any type that implements `Default` (or that you construct in an `new`/`boot` function).

```rust
#[derive(Default)]
struct App {
    count: i32,
    input: String,
    items: Vec<String>,
}
```

Keep state **minimal**: store only what you cannot derive on the fly. Computed values (e.g. the length of a list) belong in `view`, not in state.

---

## Messages

Messages are a plain `enum`. Each variant represents one thing that can happen in your app.

```rust
#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    InputChanged(String),
    ItemAdded,
}
```

Rules of thumb:

- Name variants after *what happened*, not *what should be done* — `ButtonClicked` rather than `DoSomething`.
- Every variant should carry only the data that `update` needs to do its job.
- Derive `Debug` and `Clone`; eNative requires `Clone`.

---

## Update

`update` is a plain function that takes a mutable reference to the state and a message, then mutates the state in response.

```rust
fn update(app: &mut App, message: Message) {
    match message {
        Message::Increment => app.count += 1,
        Message::Decrement => app.count -= 1,
        Message::InputChanged(text) => app.input = text,
        Message::ItemAdded => {
            if !app.input.is_empty() {
                app.items.push(app.input.clone());
                app.input.clear();
            }
        }
    }
}
```

`update` can optionally return a [`Task<Message>`](async.md) when it needs to kick off asynchronous work:

```rust
fn update(app: &mut App, message: Message) -> enative::Task<Message> {
    match message {
        Message::Load => enative::Task::perform(
            fetch_data(),
            Message::Loaded,
        ),
        Message::Loaded(data) => {
            app.data = Some(data);
            enative::Task::none()
        }
    }
}
```

---

## View

`view` is a pure function — it must not mutate state or produce side effects. It receives a shared reference to the state and returns an `Element`.

```rust
use enative::widget::{button, column, text, text_input};
use enative::Element;

fn view(app: &App) -> Element<'_, Message> {
    column![
        button("-").on_press(Message::Decrement),
        text(app.count).size(48),
        button("+").on_press(Message::Increment),
        text_input("Add item…", &app.input)
            .on_input(Message::InputChanged)
            .on_submit(Message::ItemAdded),
    ]
    .spacing(10)
    .padding(20)
    .into()
}
```

The `column![]` macro builds a vertical list of widgets. Calling `.into()` on any widget converts it into an `Element`.

---

## Entry Points

### `enative::run` (simplest)

Use this when you don't need subscriptions, a custom theme, or window settings:

```rust
pub fn main() -> enative::Result {
    enative::run(update, view)
}
```

eNative infers the initial state from `Default`.

### `enative::application` (full control)

Use the builder when you need more configuration:

```rust
pub fn main() -> enative::Result {
    enative::application(new, update, view)
        .title("My App")
        .theme(theme)
        .subscription(subscription)
        .run()
}

fn new() -> App {
    App::default()
}

fn theme(_app: &App) -> enative::Theme {
    enative::Theme::Dark
}

fn subscription(_app: &App) -> enative::Subscription<Message> {
    enative::Subscription::none()
}
```

The `new` (sometimes called `boot`) function replaces `Default` as the way to build the initial state.

### `enative::daemon`

For headless background applications (no window):

```rust
pub fn main() -> enative::Result {
    enative::daemon(new, update).run()
}
```

---

## Splitting a Large App

As your application grows you can split state, messages, update, and view into separate modules. The key insight is that `Element` and `Message` are generic, so sub-components can have their own local `Message` types and map them up to the parent:

```rust
// src/counter.rs
pub struct Counter { value: i32 }

#[derive(Debug, Clone)]
pub enum Message { Increment, Decrement }

impl Counter {
    pub fn update(&mut self, message: Message) { /* … */ }
    pub fn view(&self) -> enative::Element<'_, Message> { /* … */ }
}

// src/main.rs
enum AppMessage {
    Counter(counter::Message),
    // other variants…
}

fn update(app: &mut App, message: AppMessage) {
    match message {
        AppMessage::Counter(msg) => app.counter.update(msg),
    }
}

fn view(app: &App) -> enative::Element<'_, AppMessage> {
    app.counter
        .view()
        .map(AppMessage::Counter)
        .into()
}
```

`Element::map` translates the inner message type to an outer one without any allocation.

---

## Further Reading

- [Widgets](widgets.md) — the building blocks of `view`
- [Layout](layout.md) — arranging widgets
- [Async — Tasks & Subscriptions](async.md) — extending `update` with async work
