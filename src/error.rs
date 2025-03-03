//! Defines a common error type to use for all request handlers.

use axum::body::Body;
use axum::http::{Method, Response, StatusCode, Uri};
use axum::response::IntoResponse;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn fallback_handler_404(method: Method, uri: Uri) -> Error {
    Error::NotFound(format!("Method '{method}' not allowed for '{uri}'"))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Return `404 Not Found`
    #[error("Page not found")]
    NotFound(String),

    /// Return `500 Not Found`
    #[error("Internal Server Error")]
    Io(#[from] std::io::Error),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        match self {
            Self::NotFound(ref error_message) => {
                tracing::debug!("User error ({}): {}", self.status_code(), error_message);
                return (self.status_code(), error_message.clone()).into_response();
            }
            Self::Io(ref error_message) => {
                tracing::debug!(
                    "Internal Server Error ({}): {}",
                    self.status_code(),
                    error_message
                );
                return (self.status_code(), error_message.to_string()).into_response();
            }
        }
    }
}
