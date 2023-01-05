use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use std::error::Error;

#[derive(Debug)]
pub enum AppError {
    Conflict,
    Generic(Box<dyn Error + Send + Sync>),
    NotFound,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Conflict => (StatusCode::CONFLICT, "Already exists"),
            AppError::Generic(_error) => (StatusCode::INTERNAL_SERVER_ERROR, "Generic error"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
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

#[derive(Debug, Serialize)]
pub struct ResponseError {
    pub error: String,
}
