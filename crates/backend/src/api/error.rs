//! Backend unified error type.

use axum::{http::StatusCode, response::IntoResponse, Json};

/// API unified error type.
#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Internal(anyhow::Error),
    BadGateway(String),
}

impl AppError {
    fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadGateway(_) => StatusCode::BAD_GATEWAY,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::BadRequest(msg)
            | Self::Unauthorized(msg)
            | Self::Forbidden(msg)
            | Self::NotFound(msg)
            | Self::BadGateway(msg) => msg.clone(),
            Self::Internal(_) => "Internal server error".to_string(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Internal(anyhow::Error::new(err))
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        if let Self::Internal(e) = &self {
            tracing::error!(error = %e, "API request failed with internal error");
        }
        (
            self.status(),
            Json(serde_json::json!({ "error": self.message() })),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_and_message() {
        let err = AppError::BadRequest("bad payload".into());
        assert_eq!(err.status(), StatusCode::BAD_REQUEST);
        assert_eq!(err.message(), "bad payload");

        let internal = AppError::Internal(anyhow::anyhow!("db error"));
        assert_eq!(internal.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(internal.message(), "Internal server error");
    }
}
