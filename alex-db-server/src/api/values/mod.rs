use crate::error::AppError;
use alex_db_lib::{db::Db, db_record::DbRecord};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

#[axum_macros::debug_handler]
pub async fn create(
    State(db): State<Arc<Db>>,
    Json(input): Json<DbRecord>,
) -> Result<impl IntoResponse, AppError> {
    let key = input.key.clone();
    let value = db.try_select(&key)?;

    match value {
        None => {
            let value = db.try_insert(key, input)?.ok_or(AppError::Conflict)?;

            Ok((StatusCode::CREATED, Json(value)).into_response())
        }
        Some(_value) => Err(AppError::Conflict),
    }
}

#[axum_macros::debug_handler]
pub async fn delete(
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    db.try_select(&key)?.ok_or(AppError::NotFound)?;
    db.try_delete(&key)?;

    Ok((StatusCode::NO_CONTENT, ()).into_response())
}

#[axum_macros::debug_handler]
pub async fn list(State(db): State<Arc<Db>>) -> Result<impl IntoResponse, AppError> {
    let values = db.select_all()?;

    Ok((StatusCode::OK, Json(values)).into_response())
}

#[axum_macros::debug_handler]
pub async fn read(
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let value = db.try_select(&key)?.ok_or(AppError::NotFound)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}

#[axum_macros::debug_handler]
pub async fn update(
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
    Json(input): Json<DbRecord>,
) -> Result<impl IntoResponse, AppError> {
    db.try_select(&key)?.ok_or(AppError::NotFound)?;

    let value = db.try_upsert(key, input)?.ok_or(AppError::Conflict)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}
