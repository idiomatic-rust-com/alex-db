use crate::error::AppError;
use alex_db_lib::{
    db::Db,
    value_record::{ValuePost, ValuePut},
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

    if key != input.key {
        return Err(AppError::Conflict);
    }

    let value = db.try_upsert(key, input)?.ok_or(AppError::Conflict)?;

    Ok((StatusCode::OK, Json(value)).into_response())
}

#[cfg(test)]
mod tests {
    use crate::{app, config::Config};
    use alex_db_lib::value_record::ValueResponse;
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
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();

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
    async fn create_409() {
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();
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
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();
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
    async fn delete_404() {
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();
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
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();
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
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();
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
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();

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
    async fn read_200() {
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();
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
    async fn read_404() {
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();

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
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();
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
    async fn update_404() {
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();

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
        let config = Config {
            data_dir: None,
            port: 8080,
        };
        let router = app::get_app(config).await.unwrap();
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
