use crate::error::AppError;
use alex_db_lib::{
    db::Db,
    db_record::{ValuePost, ValuePut},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

#[axum_macros::debug_handler]
#[utoipa::path(
    post,
    path = "/values",
    request_body = ValuePost,
    responses(
        (status = 201, description = "Create value", body = ValueResponse),
    )
)]
pub async fn create(
    State(db): State<Arc<Db>>,
    Json(input): Json<ValuePost>,
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
#[utoipa::path(
    delete,
    params(
        ("key" = String, Path, description = "Key")
    ),
    path = "/values/:key",
    responses(
        (status = 204, description = "Delete value"),
        (status = 404, description = "Key not found", body = ResponseError),
    )
)]
pub async fn delete(
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    db.try_select(&key)?.ok_or(AppError::NotFound)?;
    db.try_delete(&key)?;

    Ok((StatusCode::NO_CONTENT, ()).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "/values",
    responses(
        (status = 200, description = "List of the values", body = [ValueResponse]),
    )
)]
pub async fn list(State(db): State<Arc<Db>>) -> Result<impl IntoResponse, AppError> {
    let values = db.select_all()?;

    Ok((StatusCode::OK, Json(values)).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    params(
        ("key" = String, Path, description = "Key")
    ),
    path = "/values/:key",
    responses(
        (status = 200, description = "Read value", body = ValueResponse),
        (status = 404, description = "Key not found", body = ResponseError),
    )
)]
pub async fn read(
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let value = db.try_select(&key)?.ok_or(AppError::NotFound)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    post,
    params(
        ("key" = String, Path, description = "Key")
    ),
    path = "/values/:key",
    request_body = ValuePut,
    responses(
        (status = 200, description = "Update value", body = ValueResponse),
        (status = 404, description = "Key not found", body = ResponseError),
    )
)]
pub async fn update(
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
    Json(input): Json<ValuePut>,
) -> Result<impl IntoResponse, AppError> {
    db.try_select(&key)?.ok_or(AppError::NotFound)?;

    let value = db.try_upsert(key, input)?.ok_or(AppError::Conflict)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}
