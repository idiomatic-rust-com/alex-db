use crate::error::AppError;
use alex_db_lib::{
    db::Db,
    db_record::DbRecord,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

#[axum_macros::debug_handler]
pub async fn get(
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let value = db.get(&key)?;

    Ok((StatusCode::CREATED, Json(value)).into_response())
}

#[axum_macros::debug_handler]
pub async fn post(
    State(db): State<Arc<Db>>,
    Json(input): Json<DbRecord>,
) -> Result<impl IntoResponse, AppError> {
    let value = db.insert(input.key.clone(), input)?;

    Ok((StatusCode::CREATED, Json(value)).into_response())
}
