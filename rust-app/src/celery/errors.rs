use thiserror::Error;

#[derive(Error, Debug)]
pub enum CeleryError {
    #[error("The Redis connection failed for reason: {0}")]
    ConnectionFailed(String),

    #[error("Unable to fetch next task for reason: {0}")]
    TaskFetchError(String),
}
