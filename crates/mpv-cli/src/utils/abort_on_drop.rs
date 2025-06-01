use tracing::{instrument, trace};

/// this task is no longer being polled when the handle goes out of scope
pub struct AbortOnDrop<T>(pub tokio::task::JoinHandle<T>);

impl<T> Drop for AbortOnDrop<T> {
    #[instrument(skip_all)]
    fn drop(&mut self) {
        trace!("task went out of scope");
        self.0.abort();
    }
}

impl<T> futures::Future for AbortOnDrop<T> {
    type Output = std::result::Result<T, tokio::task::JoinError>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::pin::Pin::new(&mut self.0).poll(cx)
    }
}

pub trait AbortOnDropExt<T> {
    fn abort_on_drop(self) -> AbortOnDrop<T>;
}

impl<T> AbortOnDropExt<T> for tokio::task::JoinHandle<T> {
    fn abort_on_drop(self) -> AbortOnDrop<T> {
        AbortOnDrop(self)
    }
}
