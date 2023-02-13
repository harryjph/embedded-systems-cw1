use async_trait::async_trait;
use tokio::task::{JoinError, JoinHandle};

#[async_trait]
pub trait GetJoinHandleResult<T> {
    /// Gets the result of a `JoinHandle<T>` if the join handle has finished.
    /// Returns `None` **immediately** if the join handle has not finished.
    async fn get_result(&mut self) -> Option<T>;
}

#[async_trait]
impl<T: Send> GetJoinHandleResult<Result<T, JoinError>> for JoinHandle<T> {
    async fn get_result(&mut self) -> Option<Result<T, JoinError>> {
        if self.is_finished() {
            Some(self.await)
        } else {
            None
        }
    }
}
