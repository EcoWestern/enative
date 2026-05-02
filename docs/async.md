# Async — Tasks & Subscriptions

eNative has first-class support for asynchronous operations. There are two complementary primitives:

| Primitive | When to use |
|---|---|
| **`Task`** | One-shot async work triggered from `update` (e.g. fetch data, write a file) |
| **`Subscription`** | Long-lived event streams that run for as long as your app needs them (e.g. timers, WebSocket connections, keyboard events) |

---

## Tasks

A `Task<Message>` is a future (or stream) that eventually produces one or more `Message` values, which are then fed back into `update`.

### Returning a `Task` from `update`

Change `update`'s return type from `()` to `Task<Message>`:

```rust
use enative::Task;

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::DoSomethingAsync => {
            // kick off the work and map the result to a message
            Task::perform(some_async_fn(), Message::ResultReceived)
        }
        Message::ResultReceived(result) => {
            state.result = Some(result);
            Task::none()   // nothing async to do
        }
        _ => Task::none(),
    }
}
```

When you have no async work to do, return `Task::none()`.

### `Task::perform`

The most common constructor. Runs a single future and maps its output to a message:

```rust
Task::perform(
    fetch_user(id),        // async fn fetch_user(id: u64) -> User
    Message::UserFetched,  // fn(User) -> Message
)
```

### `Task::run`

Runs a stream (an async iterator) and emits a message for each item:

```rust
use futures::stream;

Task::run(
    stream::iter(vec![1, 2, 3]),
    Message::ItemReceived,
)
```

### Chaining and combining tasks

```rust
// Map the message type
task.map(|inner_msg| OuterMessage::Inner(inner_msg))

// Run two tasks concurrently
Task::batch(vec![task_a, task_b])

// Run two tasks sequentially
task_a.chain(task_b)

// Ignore the result
task.discard()

// Make a task cancellable
let (task, handle) = task.abortable();
// Call handle.abort() later to cancel it
```

### Built-in tasks

Several modules expose helper functions that return tasks:

```rust
use enative::window;
use enative::widget::text_input;
use enative::clipboard;

// Window operations
window::resize(id, new_size).map(|_| Message::Resized)
window::close(id)

// Focus a specific text input
text_input::focus(my_input_id)

// Read / write the clipboard
clipboard::read().map(Message::ClipboardRead)
clipboard::write("Hello!".to_string())
```

---

## Subscriptions

A `Subscription<Message>` is a **declarative description** of a long-running event source. The runtime starts it when your `subscription` function includes it and stops it when it's removed — just like how `view` dictates which widgets are visible.

### Setting up subscriptions

Provide a `subscription` function via the `application` builder:

```rust
use enative::Subscription;

pub fn main() -> enative::Result {
    enative::application(new, update, view)
        .subscription(subscription)
        .run()
}

fn subscription(state: &State) -> Subscription<Message> {
    // Return Subscription::none() when nothing is needed
    if state.is_running {
        some_subscription()
    } else {
        Subscription::none()
    }
}
```

### Time — `time::every`

Emit a message at a regular interval:

```rust
use enative::time;
use std::time::Duration;

fn subscription(_state: &State) -> Subscription<Message> {
    time::every(Duration::from_millis(16)).map(|_instant| Message::Tick)
}
```

### Keyboard events — `keyboard::listen`

```rust
use enative::keyboard;

fn subscription(_state: &State) -> Subscription<Message> {
    keyboard::listen().map(Message::KeyboardEvent)
}
```

In `update`, pattern-match on `enative::keyboard::Event`:

```rust
use enative::keyboard::{self, Event, Key};

Message::KeyboardEvent(Event::KeyPressed { key: Key::Named(keyboard::key::Named::Escape), .. }) => {
    state.modal_open = false;
    Task::none()
}
```

### Window events — `window::resize_events` and friends

```rust
use enative::window;

fn subscription(_state: &State) -> Subscription<Message> {
    window::resize_events().map(|(_id, size)| Message::WindowResized(size))
}
```

### WebSockets and custom streams

Use `Subscription::run` to wrap any `Stream` as a subscription:

```rust
use enative::Subscription;
use futures::stream::Stream;

fn my_websocket_subscription() -> Subscription<Message> {
    Subscription::run(my_ws_stream, Message::WsEvent)
}
```

For a complete example, see [`examples/websocket`](../examples/websocket).

### Combining multiple subscriptions

```rust
use enative::Subscription;

fn subscription(state: &State) -> Subscription<Message> {
    Subscription::batch(vec![
        time_subscription(),
        keyboard_subscription(),
        state.connection.as_ref().map(ws_subscription).unwrap_or(Subscription::none()),
    ])
}
```

---

## Async Executors

By default eNative uses its own **thread-pool** executor. You can switch to an alternative by enabling a Cargo feature:

| Feature | Executor |
|---|---|
| `thread-pool` (default) | Built-in thread pool |
| `tokio` | [Tokio](https://tokio.rs) |
| `smol` | [Smol](https://github.com/smol-rs/smol) |

```toml
[dependencies]
enative = { version = "0.15", default-features = false, features = ["wgpu", "tiny-skia", "tokio"] }
```

With the `tokio` feature enabled you can use `tokio::spawn`, Tokio's `time`, and the full Tokio ecosystem inside your async functions.

---

## Error Handling

Async functions used with `Task::perform` can return `Result` types. Map the result to a `Message` variant that carries a `Result`:

```rust
#[derive(Debug, Clone)]
enum Message {
    FetchDone(Result<String, String>),   // use String for serialisable errors
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Fetch => Task::perform(
            async { fetch().await.map_err(|e| e.to_string()) },
            Message::FetchDone,
        ),
        Message::FetchDone(Ok(data)) => {
            state.data = Some(data);
            Task::none()
        }
        Message::FetchDone(Err(error)) => {
            state.error = Some(error);
            Task::none()
        }
    }
}
```

---

## Full Example — Download with Progress

```rust
use enative::{Task, Subscription};
use enative::widget::{button, column, progress_bar, text};
use enative::Element;

#[derive(Default)]
struct App {
    progress: f32,
    downloading: bool,
}

#[derive(Debug, Clone)]
enum Message {
    StartDownload,
    Progress(f32),
    Done,
}

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::StartDownload => {
            app.downloading = true;
            app.progress = 0.0;
            Task::none()
        }
        Message::Progress(p) => {
            app.progress = p;
            Task::none()
        }
        Message::Done => {
            app.downloading = false;
            app.progress = 1.0;
            Task::none()
        }
    }
}

fn subscription(app: &App) -> Subscription<Message> {
    if app.downloading {
        download_stream().map(|event| match event {
            DownloadEvent::Progress(p) => Message::Progress(p),
            DownloadEvent::Done => Message::Done,
        })
    } else {
        Subscription::none()
    }
}

fn view(app: &App) -> Element<'_, Message> {
    column![
        progress_bar(0.0..=1.0, app.progress),
        button("Download").on_press(Message::StartDownload),
        text(if app.downloading { "Downloading…" } else { "Ready" }),
    ]
    .spacing(12)
    .padding(20)
    .into()
}
```

See the full runnable version in [`examples/download_progress`](../examples/download_progress).

---

## Further Reading

- [Architecture](architecture.md) — how `update` and the message loop work
- [`examples/websocket`](../examples/websocket) — WebSocket subscription
- [`examples/stopwatch`](../examples/stopwatch) — `time::every` subscription
- [`examples/download_progress`](../examples/download_progress) — streaming progress
- [`examples/pokedex`](../examples/pokedex) — `Task::perform` for HTTP requests
