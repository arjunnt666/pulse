use thiserror::Error;

pub type Result<T> = std::result::Result<T, PulseError>;

#[derive(Debug, Error)]
pub enum PulseError {
    #[error("client not found: {0}")] ClientNotFound(String),
    #[error("entity not found: {0}")] EntityNotFound(String),
    #[error("room not found: {0}")] RoomNotFound(String),
    #[error("transport error: {0}")] Transport(String),
    #[error("protocol error: {0}")] Protocol(String),
    #[error("invalid state: {0}")] InvalidState(String),
    #[error("internal: {0}")] Internal(String),
}
