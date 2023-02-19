use crate::{access::Access, error::AppError};
use alex_db_lib::{
    db::{Db, Direction, Sort},
    value_record::{
        ValueAppend, ValueDecrement, ValueIncrement, ValuePopBack, ValuePopFront, ValuePost,
        ValuePrepend, ValuePut,
    },
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::sync::Arc;
use validator::Validate;

mod test;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub direction: Option<Direction>,
    pub ends_at: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
    pub page: Option<usize>,
    pub sort: Option<Sort>,
    pub starts_at: Option<DateTime<Utc>>,
}

#[axum_macros::debug_handler]
#[utoipa::path(
    put,
    params(
        ("key" = String, Path, description = "Value key.")
    ),
    path = "/values/:key/append",
    request_body = ValueAppend,
    responses(
        (status = 200, description = "Value appended.", body = ValueResponse),
        (status = 401, description = "Unauthorized request.", body = ResponseError),
        (status = 404, description = "Value not found by key.", body = ResponseError),
        (status = 409, description = "Conflicting request.", body = ResponseError),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
pub async fn append(
    access: Access,
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
    Json(input): Json<ValueAppend>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    input.validate()?;

    db.try_read(&key)?.ok_or(AppError::NotFound)?;

    let value = db.try_append(&key, input)?.ok_or(AppError::Conflict)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    post,
    path = "/values",
    request_body = ValuePost,
    responses(
        (status = 201, description = "Value created.", body = ValueResponse),
        (status = 401, description = "Unauthorized request.", body = ResponseError),
        (status = 409, description = "Conflicting request.", body = ResponseError),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
pub async fn create(
    access: Access,
    State(db): State<Arc<Db>>,
    Json(input): Json<ValuePost>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    input.validate()?;
    let key = input.key.clone();
    let value = db.try_read(&key)?;

    match value {
        None => {
            let value = db.try_create(input)?.ok_or(AppError::Conflict)?;

            Ok((StatusCode::CREATED, Json(value)).into_response())
        }
        Some(_value) => Err(AppError::Conflict),
    }
}

#[axum_macros::debug_handler]
#[utoipa::path(
    put,
    params(
        ("key" = String, Path, description = "Value key.")
    ),
    path = "/values/:key/decrement",
    request_body = ValueDecrement,
    responses(
        (status = 200, description = "Value decremented.", body = ValueResponse),
        (status = 401, description = "Unauthorized request.", body = ResponseError),
        (status = 404, description = "Value not found by key.", body = ResponseError),
        (status = 409, description = "Conflicting request.", body = ResponseError),
        (status = 422, description = "Unprocessable entity."),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
pub async fn decrement(
    access: Access,
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
    Json(input): Json<ValueDecrement>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    input.validate()?;

    db.try_read(&key)?.ok_or(AppError::NotFound)?;

    let value = db.try_decrement(&key, input)?.ok_or(AppError::Conflict)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    delete,
    params(
        ("key" = String, Path, description = "Value key.")
    ),
    path = "/values/:key",
    responses(
        (status = 204, description = "Value deleted."),
        (status = 401, description = "Unauthorized request.", body = ResponseError),
        (status = 404, description = "Value not found by key.", body = ResponseError),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
pub async fn delete(
    access: Access,
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    db.try_read(&key)?.ok_or(AppError::NotFound)?;
    db.try_delete(&key)?;

    Ok((StatusCode::NO_CONTENT, ()).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    put,
    params(
        ("key" = String, Path, description = "Value key.")
    ),
    path = "/values/:key/increment",
    request_body = ValueIncrement,
    responses(
        (status = 200, description = "Value incremented.", body = ValueResponse),
        (status = 401, description = "Unauthorized request.", body = ResponseError),
        (status = 404, description = "Value not found by key.", body = ResponseError),
        (status = 409, description = "Conflicting request.", body = ResponseError),
        (status = 422, description = "Unprocessable entity."),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
pub async fn increment(
    access: Access,
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
    Json(input): Json<ValueIncrement>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    input.validate()?;

    db.try_read(&key)?.ok_or(AppError::NotFound)?;

    let value = db.try_increment(&key, input)?.ok_or(AppError::Conflict)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "/values",
    responses(
        (status = 200, description = "List of values.", body = [ValueResponse]),
        (status = 401, description = "Unauthorized request.", body = ResponseError),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
pub async fn list(
    access: Access,
    State(db): State<Arc<Db>>,
    query_params: Query<QueryParams>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    let Query(query_params) = query_params;
    let direction = query_params.direction.unwrap_or(Direction::Asc);
    let sort = query_params.sort.unwrap_or(Sort::CreatedAt);

    let values = db.list(direction, query_params.limit, query_params.page, sort)?;

    Ok((StatusCode::OK, Json(values)).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    put,
    params(
        ("key" = String, Path, description = "Value key.")
    ),
    path = "/values/:key/pop-back",
    request_body = ValuePopBack,
    responses(
        (status = 200, description = "Value prepended.", body = [Value]),
        (status = 401, description = "Unauthorized request.", body = ResponseError),
        (status = 404, description = "Value not found by key.", body = ResponseError),
        (status = 409, description = "Conflicting request.", body = ResponseError),
        (status = 422, description = "Unprocessable entity."),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
pub async fn pop_back(
    access: Access,
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
    Json(input): Json<ValuePopBack>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    input.validate()?;

    db.try_read(&key)?.ok_or(AppError::NotFound)?;

    let values = db.try_pop_back(&key, input)?.ok_or(AppError::Conflict)?;

    Ok((StatusCode::OK, Json(values)).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    put,
    params(
        ("key" = String, Path, description = "Value key.")
    ),
    path = "/values/:key/pop-front",
    request_body = ValuePopFront,
    responses(
        (status = 200, description = "Value prepended.", body = [Value]),
        (status = 401, description = "Unauthorized request.", body = ResponseError),
        (status = 404, description = "Value not found by key.", body = ResponseError),
        (status = 409, description = "Conflicting request.", body = ResponseError),
        (status = 422, description = "Unprocessable entity."),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
pub async fn pop_front(
    access: Access,
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
    Json(input): Json<ValuePopFront>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    input.validate()?;

    db.try_read(&key)?.ok_or(AppError::NotFound)?;

    let values = db.try_pop_front(&key, input)?.ok_or(AppError::Conflict)?;

    Ok((StatusCode::OK, Json(values)).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    put,
    params(
        ("key" = String, Path, description = "Value key.")
    ),
    path = "/values/:key/prepend",
    request_body = ValuePrepend,
    responses(
        (status = 200, description = "Value prepended.", body = ValueResponse),
        (status = 401, description = "Unauthorized request.", body = ResponseError),
        (status = 404, description = "Value not found by key.", body = ResponseError),
        (status = 409, description = "Conflicting request.", body = ResponseError),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
pub async fn prepend(
    access: Access,
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
    Json(input): Json<ValuePrepend>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    input.validate()?;

    db.try_read(&key)?.ok_or(AppError::NotFound)?;

    let value = db.try_prepend(&key, input)?.ok_or(AppError::Conflict)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    params(
        ("key" = String, Path, description = "Value key.")
    ),
    path = "/values/:key",
    responses(
        (status = 200, description = "Value read.", body = ValueResponse),
        (status = 401, description = "Unauthorized request.", body = ResponseError),
        (status = 404, description = "Value not found by key.", body = ResponseError),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
pub async fn read(
    access: Access,
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    let value = db.try_read(&key)?.ok_or(AppError::NotFound)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}

#[axum_macros::debug_handler]
#[utoipa::path(
    put,
    params(
        ("key" = String, Path, description = "Value key.")
    ),
    path = "/values/:key",
    request_body = ValuePut,
    responses(
        (status = 200, description = "Value updated.", body = ValueResponse),
        (status = 401, description = "Unauthorized request.", body = ResponseError),
        (status = 404, description = "Value not found by key.", body = ResponseError),
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
pub async fn update(
    access: Access,
    State(db): State<Arc<Db>>,
    Path(key): Path<String>,
    Json(input): Json<ValuePut>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    input.validate()?;

    db.try_read(&key)?.ok_or(AppError::NotFound)?;

    let value = db.try_update(&key, input)?.ok_or(AppError::Conflict)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}
