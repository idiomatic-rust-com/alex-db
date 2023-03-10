use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use http::header::ToStrError;
use serde::Serialize;
use serde_json::json;
use std::error::Error;
use utoipa::ToSchema;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    Conflict,
    Generic(Box<dyn Error + Send + Sync>),
    Header(ToStrError),
    NotFound,
    Unauthorized,
    Uuid(uuid::Error),
    Validation(ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Conflict => (StatusCode::CONFLICT, "Conflicting request."),
            AppError::Generic(_error) => (StatusCode::INTERNAL_SERVER_ERROR, "Generic error."),
            AppError::Header(_error) => (StatusCode::BAD_REQUEST, "Invalid header."),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found."),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized request."),
            AppError::Uuid(_error) => (StatusCode::BAD_REQUEST, "Invalid API key."),
            AppError::Validation(_error) => (StatusCode::BAD_REQUEST, "Invalid key."),
        };

        let body = Json(json!(ResponseError {
            error: error_message.to_string(),
        }));

        (status, body).into_response()
    }
}

impl From<Box<dyn Error + Send + Sync>> for AppError {
    fn from(inner: Box<dyn Error + Send + Sync>) -> Self {
        AppError::Generic(inner)
    }
}

impl From<ToStrError> for AppError {
    fn from(inner: ToStrError) -> Self {
        AppError::Header(inner)
    }
}

impl From<uuid::Error> for AppError {
    fn from(inner: uuid::Error) -> Self {
        AppError::Uuid(inner)
    }
}

impl From<ValidationErrors> for AppError {
    fn from(inner: ValidationErrors) -> Self {
        AppError::Validation(inner)
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseError {
    pub error: String,
}
