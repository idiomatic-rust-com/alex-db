use crate::{access::Access, error::AppError};
use alex_db_lib::{
    db::{Db, Direction, Sort},
    value_record::{
        ValueAppend, ValueDecrement, ValueIncrement, ValuePost, ValuePrepend, ValuePut,
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

    db.try_select(&key)?.ok_or(AppError::NotFound)?;

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
        (status = 422, description = "Unprocessable entity."),
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
    let value = db.try_select(&key)?;

    match value {
        None => {
            let value = db.try_insert(input)?.ok_or(AppError::Conflict)?;

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

    db.try_select(&key)?.ok_or(AppError::NotFound)?;

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

    db.try_select(&key)?.ok_or(AppError::NotFound)?;
    db.try_delete_by_key(&key)?;

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

    db.try_select(&key)?.ok_or(AppError::NotFound)?;

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

    let values = db.select_all(direction, query_params.limit, query_params.page, sort)?;

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

    db.try_select(&key)?.ok_or(AppError::NotFound)?;

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

    let value = db.try_select(&key)?.ok_or(AppError::NotFound)?;

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
        (status = 409, description = "Conflicting request.", body = ResponseError),
        (status = 422, description = "Unprocessable entity."),
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
    if key != input.key {
        return Err(AppError::Conflict);
    }

    db.try_select(&key)?.ok_or(AppError::NotFound)?;

    let value = db.try_upsert(input)?.ok_or(AppError::Conflict)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}

#[cfg(test)]
mod tests {
    use crate::{app, config::Config};
    use alex_db_lib::{
        config::Config as DbConfig,
        value_record::{Value, ValueResponse},
    };
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use fake::{
        faker::lorem::en::{Paragraph, Word},
        Fake, Faker,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn append_200_array_array_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: bool = Faker.fake();
        let value_array = vec![vec![value]];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::Boolean(value)])])
        );

        let append_value: bool = Faker.fake();
        let append_value_array = vec![vec![append_value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![
                Value::Array(vec![Value::Boolean(value)]),
                Value::Array(vec![Value::Boolean(append_value)])
            ])
        );
    }

    #[tokio::test]
    async fn append_200_array_array_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: i64 = Faker.fake();
        let value_array = vec![vec![value]];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::Integer(value)])])
        );

        let append_value: i64 = Faker.fake();
        let append_value_array = vec![vec![append_value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![
                Value::Array(vec![Value::Integer(value)]),
                Value::Array(vec![Value::Integer(append_value)])
            ])
        );
    }

    #[tokio::test]
    async fn append_200_array_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: bool = Faker.fake();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Boolean(value)]));

        let append_value: bool = Faker.fake();
        let append_value_array = vec![append_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Boolean(value), Value::Boolean(append_value)])
        );
    }

    #[tokio::test]
    async fn append_200_array_boolean_and_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: bool = Faker.fake();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Boolean(value)]));

        let append_value: i64 = Faker.fake();
        let append_value_array = vec![append_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Boolean(value), Value::Integer(append_value)])
        );
    }

    #[tokio::test]
    async fn append_200_array_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: i64 = Faker.fake();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Integer(value)]));

        let append_value: i64 = Faker.fake();
        let append_value_array = vec![append_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Integer(value), Value::Integer(append_value)])
        );
    }

    #[tokio::test]
    async fn append_200_array_integer_and_string() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: i64 = Faker.fake();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Integer(value)]));

        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Integer(value), Value::String(append_value)])
        );
    }

    #[tokio::test]
    async fn append_200_array_string() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value.clone())]));

        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::String(value), Value::String(append_value)])
        );
    }

    #[tokio::test]
    async fn append_200_array_string_and_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value.clone())]));

        let append_value: bool = Faker.fake();
        let append_value_array = vec![append_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::String(value), Value::Boolean(append_value)])
        );
    }

    #[tokio::test]
    async fn append_200_authentication() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value.clone())]));

        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::String(value), Value::String(append_value)])
        );
    }

    #[tokio::test]
    async fn append_401() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value)]));

        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn append_404() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/decrement", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn append_409() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &append_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn append_422() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value)]));

        let value: f64 = Faker.fake();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/append", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "append": &value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn create_201_array_array_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value: bool = Faker.fake();
        let value_array = vec![vec![value]];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::Boolean(value)])])
        );
    }

    #[tokio::test]
    async fn create_201_array_array_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value: i64 = Faker.fake();
        let value_array = vec![vec![value]];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::Integer(value)])])
        );
    }

    #[tokio::test]
    async fn create_201_array_array_string() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![vec![value.clone()]];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::String(value)])])
        );
    }

    #[tokio::test]
    async fn create_201_array_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value: bool = Faker.fake();
        let value_array = vec![value];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Boolean(value)]));
    }

    #[tokio::test]
    async fn create_201_array_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value: i64 = Faker.fake();
        let value_array = vec![value];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Integer(value)]));
    }

    #[tokio::test]
    async fn create_201_array_string() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value)]));
    }

    #[tokio::test]
    async fn create_201_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = Faker.fake();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Boolean(value));
    }

    #[tokio::test]
    async fn create_201_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = Faker.fake();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));
    }

    #[tokio::test]
    async fn create_201_string() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));
    }

    #[tokio::test]
    async fn create_201_authentication() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));
    }

    #[tokio::test]
    async fn create_401() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn create_409() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let value = Paragraph(2..10).fake::<String>();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn create_422() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value: f64 = Faker.fake();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn decrement_200() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 100;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let decrement_value: i64 = 50;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/decrement", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "decrement": &decrement_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value - decrement_value.abs()));
    }

    #[tokio::test]
    async fn decrement_200_authentication() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 100;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let decrement_value: i64 = 50;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/decrement", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({ "decrement": &decrement_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value - decrement_value.abs()));
    }

    #[tokio::test]
    async fn decrement_200_no_decrement_value() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 100;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/decrement", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::json!({}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value - 1));
    }

    #[tokio::test]
    async fn decrement_401() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 100;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let decrement_value: i64 = 50;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/decrement", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "decrement": &decrement_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn decrement_404() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let decrement_value: i64 = 50;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/decrement", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "decrement": &decrement_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn decrement_409() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let decrement_value: i64 = 50;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/decrement", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "decrement": &decrement_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn decrement_422() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 100;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/decrement", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "decrement": "wrong_value"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn delete_204() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn delete_204_authentication() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn delete_401() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn delete_404() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();
        let second_cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);

        let response = second_cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::DELETE)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn increment_200() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 100;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let increment_value: i64 = 50;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/increment", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "increment": &increment_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value + increment_value.abs()));
    }

    #[tokio::test]
    async fn increment_200_authentication() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 100;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let increment_value: i64 = 50;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/increment", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({ "increment": &increment_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value + increment_value.abs()));
    }

    #[tokio::test]
    async fn increment_200_no_increment_value() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 100;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/increment", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::json!({}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value + 1));
    }

    #[tokio::test]
    async fn increment_401() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 100;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let increment_value: i64 = 50;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/increment", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "increment": &increment_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn increment_404() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let increment_value: i64 = 50;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/increment", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "increment": &increment_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn increment_409() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let increment_value: i64 = 50;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/increment", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "increment": &increment_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn increment_422() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 100;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/increment", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "increment": "wrong_value"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn list_one_200() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<ValueResponse> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.len(), 1);
    }

    #[tokio::test]
    async fn list_two_200() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();
        let second_cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let response = second_cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<ValueResponse> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.len(), 2);
    }

    #[tokio::test]
    async fn list_empty_200() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<ValueResponse> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.len(), 0);
    }

    #[tokio::test]
    async fn list_empty_200_authentication() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<ValueResponse> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.len(), 0);
    }

    #[tokio::test]
    async fn list_empty_401() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn prepend_200_array_array_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: bool = Faker.fake();
        let value_array = vec![vec![value]];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::Boolean(value)])])
        );

        let prepend_value: bool = Faker.fake();
        let prepend_value_array = vec![vec![prepend_value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![
                Value::Array(vec![Value::Boolean(prepend_value)]),
                Value::Array(vec![Value::Boolean(value)])
            ])
        );
    }

    #[tokio::test]
    async fn prepend_200_array_array_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: i64 = Faker.fake();
        let value_array = vec![vec![value]];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::Integer(value)])])
        );

        let prepend_value: i64 = Faker.fake();
        let prepend_value_array = vec![vec![prepend_value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![
                Value::Array(vec![Value::Integer(prepend_value)]),
                Value::Array(vec![Value::Integer(value)])
            ])
        );
    }

    #[tokio::test]
    async fn prepend_200_array_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: bool = Faker.fake();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Boolean(value)]));

        let prepend_value: bool = Faker.fake();
        let prepend_value_array = vec![prepend_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Boolean(prepend_value), Value::Boolean(value)])
        );
    }

    #[tokio::test]
    async fn prepend_200_array_boolean_and_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: bool = Faker.fake();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Boolean(value)]));

        let prepend_value: i64 = Faker.fake();
        let prepend_value_array = vec![prepend_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Integer(prepend_value), Value::Boolean(value)])
        );
    }

    #[tokio::test]
    async fn prepend_200_array_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: i64 = Faker.fake();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Integer(value)]));

        let prepend_value: i64 = Faker.fake();
        let prepend_value_array = vec![prepend_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Integer(prepend_value), Value::Integer(value)])
        );
    }

    #[tokio::test]
    async fn prepend_200_array_integer_and_string() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: i64 = Faker.fake();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Integer(value)]));

        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::String(prepend_value), Value::Integer(value)])
        );
    }

    #[tokio::test]
    async fn prepend_200_array_string() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value.clone())]));

        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::String(prepend_value), Value::String(value)])
        );
    }

    #[tokio::test]
    async fn prepend_200_array_string_and_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value.clone())]));

        let prepend_value: bool = Faker.fake();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Boolean(prepend_value), Value::String(value)])
        );
    }

    #[tokio::test]
    async fn prepend_200_authentication() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value.clone())]));

        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::String(prepend_value), Value::String(value)])
        );
    }

    #[tokio::test]
    async fn prepend_401() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value)]));

        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn prepend_404() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/decrement", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn prepend_409() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &prepend_value_array }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn prepend_422() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value)]));

        let value: f64 = Faker.fake();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}/prepend", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "prepend": &value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn read_200() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value.clone()));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));
    }

    #[tokio::test]
    async fn read_200_authentication() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value.clone()));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));
    }

    #[tokio::test]
    async fn read_401() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn read_404() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn update_200_array_array_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: bool = Faker.fake();
        let value_array = vec![vec![value]];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::Boolean(value)])])
        );

        let value: bool = Faker.fake();
        let value_array = vec![vec![value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::Boolean(value)])])
        );
    }

    #[tokio::test]
    async fn update_200_array_array_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: i64 = Faker.fake();
        let value_array = vec![vec![value]];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::Integer(value)])])
        );

        let value: i64 = Faker.fake();
        let value_array = vec![vec![value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::Integer(value)])])
        );
    }

    #[tokio::test]
    async fn update_200_array_array_string() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![vec![value.clone()]];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::String(value)])])
        );

        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![vec![value.clone()]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(
            body.value,
            Value::Array(vec![Value::Array(vec![Value::String(value)])])
        );
    }

    #[tokio::test]
    async fn update_200_array_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: bool = Faker.fake();
        let value_array = vec![value];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Boolean(value)]));

        let value: bool = Faker.fake();
        let value_array = vec![value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Boolean(value)]));
    }

    #[tokio::test]
    async fn update_200_array_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value: i64 = Faker.fake();
        let value_array = vec![value];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Integer(value)]));

        let value: i64 = Faker.fake();
        let value_array = vec![value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::Integer(value)]));
    }

    #[tokio::test]
    async fn update_200_array_string() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value)]));

        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value_array
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Array(vec![Value::String(value)]));
    }

    #[tokio::test]
    async fn update_200_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Faker.fake();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Boolean(value));

        let value = Faker.fake();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Boolean(value));
    }

    #[tokio::test]
    async fn update_200_boolean_to_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Faker.fake();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Boolean(value));

        let value = Faker.fake();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));
    }

    #[tokio::test]
    async fn update_200_integer() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Faker.fake();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let value = Faker.fake();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));
    }

    #[tokio::test]
    async fn update_200_integer_to_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Faker.fake();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let value = Faker.fake();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Boolean(value));
    }

    #[tokio::test]
    async fn update_200_string() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let value = Paragraph(2..10).fake::<String>();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));
    }

    #[tokio::test]
    async fn update_200_string_to_boolean() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let value = Faker.fake();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Boolean(value));
    }

    #[tokio::test]
    async fn update_200_authentication() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let value = Paragraph(2..10).fake::<String>();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));
    }

    #[tokio::test]
    async fn update_401() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = true;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let value = Paragraph(2..10).fake::<String>();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn update_404() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn update_409() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::String(value));

        let value = Paragraph(2..10).fake::<String>();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": "wrong-key",
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn update_422() {
        let mut db_config = DbConfig::default();
        db_config.enable_security_api_keys = false;
        let config = Config::new(db_config, 8080);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = Faker.fake();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/values")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Integer(value));

        let value: f64 = Faker.fake();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{}", key))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({
                            "key": &key,
                            "value": &value
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
