use crate::utils::response::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub enum AppError {
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        ApiResponse::<()>::error(status, &error_message).into_response()
    }
}
