use enative_core::clipboard::{Content, Error, Kind};
use layershellev::WindowWrapper;

pub struct LayerShellClipboard {
    state: State,
}

enum State {
    Connected(window_clipboard::Clipboard),
    Unavailable,
}

impl LayerShellClipboard {
    pub fn connect(window: &WindowWrapper) -> Self {
        #[allow(unsafe_code)]
        let state = unsafe { window_clipboard::Clipboard::connect(window) }
            .ok()
            .map(State::Connected)
            .unwrap_or(State::Unavailable);
        Self { state }
    }

    pub fn unconnected() -> Self {
        Self {
            state: State::Unavailable,
        }
    }

    pub fn read(&self, kind: Kind, callback: impl FnOnce(Result<Content, Error>)) {
        match &self.state {
            State::Connected(clipboard) => {
                let result = match kind {
                    Kind::Text => clipboard
                        .read()
                        .map(Content::Text)
                        .map_err(|_| Error::ClipboardUnavailable),
                    _ => Err(Error::ClipboardUnavailable),
                };
                callback(result);
            }
            State::Unavailable => callback(Err(Error::ClipboardUnavailable)),
        }
    }

    pub fn write(&mut self, content: Content, callback: impl FnOnce(Result<(), Error>)) {
        match &mut self.state {
            State::Connected(clipboard) => {
                let result = match content {
                    Content::Text(text) => clipboard
                        .write(text)
                        .map_err(|_| Error::ClipboardUnavailable),
                    _ => Err(Error::ClipboardUnavailable),
                };
                callback(result);
            }
            State::Unavailable => callback(Err(Error::ClipboardUnavailable)),
        }
    }
}
