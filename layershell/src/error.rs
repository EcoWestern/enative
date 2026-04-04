use enative_futures::futures;

/// An error that occurred while running a layer shell application.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The futures executor could not be created.
    #[error("the futures executor could not be created")]
    ExecutorCreationFailed(futures::io::Error),

    /// The application window could not be created.
    #[error("the application window could not be created")]
    WindowCreationFailed(Box<dyn std::error::Error + Send + Sync>),

    /// The application graphics context could not be created.
    #[error("the application graphics context could not be created")]
    GraphicsCreationFailed(enative_graphics::Error),
}

impl From<enative_graphics::Error> for Error {
    fn from(error: enative_graphics::Error) -> Error {
        Error::GraphicsCreationFailed(error)
    }
}
