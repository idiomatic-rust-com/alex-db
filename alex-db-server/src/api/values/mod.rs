use crate::{access::Access, error::AppError};
use alex_db_lib::{
    db::{Db, Direction, Sort},
    value_record::{ValuePost, ValuePut},
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
    post,
    path = "/values",
    request_body = ValuePost,
    responses(
        (status = 201, description = "Create value", body = ValueResponse),
        (status = 401, description = "Unauthorized", body = ResponseError),
        (status = 409, description = "Conflict", body = ResponseError),
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
    delete,
    params(
        ("key" = String, Path, description = "Key")
    ),
    path = "/values/:key",
    responses(
        (status = 204, description = "Delete value"),
        (status = 401, description = "Unauthorized", body = ResponseError),
        (status = 404, description = "Key not found", body = ResponseError),
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
    get,
    path = "/values",
    responses(
        (status = 200, description = "List of the values", body = [ValueResponse]),
        (status = 401, description = "Unauthorized", body = ResponseError),
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
    get,
    params(
        ("key" = String, Path, description = "Key")
    ),
    path = "/values/:key",
    responses(
        (status = 200, description = "Read value", body = ValueResponse),
        (status = 401, description = "Unauthorized", body = ResponseError),
        (status = 404, description = "Key not found", body = ResponseError),
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
    post,
    params(
        ("key" = String, Path, description = "Key")
    ),
    path = "/values/:key",
    request_body = ValuePut,
    responses(
        (status = 200, description = "Update value", body = ValueResponse),
        (status = 401, description = "Unauthorized", body = ResponseError),
        (status = 404, description = "Key not found", body = ResponseError),
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
    use alex_db_lib::{config::Config as DbConfig, value_record::ValueResponse};
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use fake::{
        faker::lorem::en::{Paragraph, Word},
        Fake,
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn create_201() {
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
        assert_eq!(body.value, value);
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
        assert_eq!(body.value, value);
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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);
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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);
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
        assert_eq!(body.value, value);

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
    async fn update_200() {
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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);
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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);
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
        assert_eq!(body.value, value);

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
        assert_eq!(body.value, value);

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
}
