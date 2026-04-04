use futures::{
    channel::mpsc,
    task::{Context, Poll},
    Sink,
};
use std::pin::Pin;
use std::sync::mpsc as stdmpsc;

/// An event loop proxy that implements `Sink`.
#[derive(Debug)]
pub struct LayershellProxy<Message: 'static>(stdmpsc::Sender<Message>);

impl<Message: 'static> Clone for LayershellProxy<Message> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Message: 'static> LayershellProxy<Message> {
    pub fn new(sender: stdmpsc::Sender<Message>) -> Self {
        Self(sender)
    }
}

impl<Message: 'static> Sink<Message> for LayershellProxy<Message> {
    type Error = mpsc::SendError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, message: Message) -> Result<(), Self::Error> {
        let _ = self.0.send(message).ok();
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}
