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
    use std::collections::VecDeque;
    use tower::ServiceExt;

    #[tokio::test]
    async fn append_200_array_array_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Boolean(value)
            ]))]))
        );

        let append_value: bool = Faker.fake();
        let append_value_array = vec![vec![append_value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::Array(VecDeque::from([Value::Boolean(value)])),
                Value::Array(VecDeque::from([Value::Boolean(append_value)]))
            ]))
        );
    }

    #[tokio::test]
    async fn append_200_array_array_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 1.5;
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Float(value)
            ]))]))
        );

        let append_value = 1.75;
        let append_value_array = vec![vec![append_value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::Array(VecDeque::from([Value::Float(value)])),
                Value::Array(VecDeque::from([Value::Float(append_value)]))
            ]))
        );
    }

    #[tokio::test]
    async fn append_200_array_array_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Integer(value)
            ]))]))
        );

        let append_value: i64 = Faker.fake();
        let append_value_array = vec![vec![append_value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::Array(VecDeque::from([Value::Integer(value)])),
                Value::Array(VecDeque::from([Value::Integer(append_value)]))
            ]))
        );
    }

    #[tokio::test]
    async fn append_200_array_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Boolean(value)]))
        );

        let append_value: bool = Faker.fake();
        let append_value_array = vec![append_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::Boolean(value),
                Value::Boolean(append_value)
            ]))
        );
    }

    #[tokio::test]
    async fn append_200_array_boolean_and_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Boolean(value)]))
        );

        let append_value: i64 = Faker.fake();
        let append_value_array = vec![append_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::Boolean(value),
                Value::Integer(append_value)
            ]))
        );
    }

    #[tokio::test]
    async fn append_200_array_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 1.5;
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Float(value)]))
        );

        let append_value = 1.75;
        let append_value_array = vec![append_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::Float(value),
                Value::Float(append_value)
            ]))
        );
    }

    #[tokio::test]
    async fn append_200_array_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Integer(value)]))
        );

        let append_value: i64 = Faker.fake();
        let append_value_array = vec![append_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::Integer(value),
                Value::Integer(append_value)
            ]))
        );
    }

    #[tokio::test]
    async fn append_200_array_float_and_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 1.5;
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Float(value)]))
        );

        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::Float(value),
                Value::String(append_value)
            ]))
        );
    }

    #[tokio::test]
    async fn append_200_array_integer_and_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Integer(value)]))
        );

        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::Integer(value),
                Value::String(append_value)
            ]))
        );
    }

    #[tokio::test]
    async fn append_200_array_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value.clone())]))
        );

        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::String(value),
                Value::String(append_value)
            ]))
        );
    }

    #[tokio::test]
    async fn append_200_array_string_and_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value.clone())]))
        );

        let append_value: bool = Faker.fake();
        let append_value_array = vec![append_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::String(value),
                Value::Boolean(append_value)
            ]))
        );
    }

    #[tokio::test]
    async fn append_200_authentication() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value.clone())]))
        );

        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
            Value::Array(VecDeque::from([
                Value::String(value),
                Value::String(append_value)
            ]))
        );
    }

    #[tokio::test]
    async fn append_401() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value)]))
        );

        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let append_value = Paragraph(2..10).fake::<String>();
        let append_value_array = vec![append_value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/append"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/append"))
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
    async fn create_201_array_array_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Boolean(value)
            ]))]))
        );
    }

    #[tokio::test]
    async fn create_201_array_array_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = 1.5;
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Float(value)
            ]))]))
        );
    }

    #[tokio::test]
    async fn create_201_array_array_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Integer(value)
            ]))]))
        );
    }

    #[tokio::test]
    async fn create_201_array_array_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::String(value)
            ]))]))
        );
    }

    #[tokio::test]
    async fn create_201_array_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Boolean(value)]))
        );
    }

    #[tokio::test]
    async fn create_201_array_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = 1.5;
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Float(value)]))
        );
    }

    #[tokio::test]
    async fn create_201_array_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Integer(value)]))
        );
    }

    #[tokio::test]
    async fn create_201_array_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value)]))
        );
    }

    #[tokio::test]
    async fn create_201_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
    async fn create_201_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = 1.5;

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
        assert_eq!(body.value, Value::Float(value));
    }

    #[tokio::test]
    async fn create_201_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
    async fn decrement_200() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/decrement"))
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
    async fn decrement_200_max() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 0;

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

        let decrement_value = i64::MAX;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/decrement"))
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
        assert_eq!(body.value, Value::Integer(i64::MIN + 1));
    }

    #[tokio::test]
    async fn decrement_200_min() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 0;

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

        let decrement_value = i64::MIN;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/decrement"))
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
        assert_eq!(body.value, Value::Integer(0));
    }

    #[tokio::test]
    async fn decrement_200_min_plus_1() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 0;

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

        let decrement_value = i64::MIN + 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/decrement"))
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
        assert_eq!(body.value, Value::Integer(i64::MIN + 1));
    }

    #[tokio::test]
    async fn decrement_200_authentication() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/decrement"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/decrement"))
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/decrement"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let decrement_value: i64 = 50;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/decrement"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/decrement"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/decrement"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
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
                    .uri(format!("/values/{key}"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/increment"))
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
    async fn increment_200_max() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 0;

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

        let increment_value = i64::MAX;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/increment"))
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
        assert_eq!(body.value, Value::Integer(i64::MAX));
    }

    #[tokio::test]
    async fn increment_200_min() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 0;

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

        let increment_value = i64::MIN;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/increment"))
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
        assert_eq!(body.value, Value::Integer(0));
    }

    #[tokio::test]
    async fn increment_200_min_plus1() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 0;

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

        let increment_value = i64::MIN + 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/increment"))
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
        assert_eq!(body.value, Value::Integer(i64::MAX));
    }

    #[tokio::test]
    async fn increment_200_authentication() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/increment"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/increment"))
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/increment"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let increment_value: i64 = 50;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/increment"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/increment"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/increment"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
    async fn pop_back_200_array_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1: bool = Faker.fake();
        let value2: bool = Faker.fake();
        let value_array = vec![value1, value2];

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
            Value::Array(VecDeque::from([
                Value::Boolean(value1),
                Value::Boolean(value2)
            ]))
        );

        let pop_back_value = 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_back": &pop_back_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::Boolean(value2)]);
    }

    #[tokio::test]
    async fn pop_back_200_array_boolean_more() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1: bool = Faker.fake();
        let value2: bool = Faker.fake();
        let value3: bool = Faker.fake();
        let value_array = vec![value1, value2, value3];

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
            Value::Array(VecDeque::from([
                Value::Boolean(value1),
                Value::Boolean(value2),
                Value::Boolean(value3)
            ]))
        );

        let pop_back_value = 2;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_back": &pop_back_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::Boolean(value3), Value::Boolean(value2)]);
    }

    #[tokio::test]
    async fn pop_back_200_array_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1 = 1.5;
        let value2 = 1.75;
        let value_array = vec![value1, value2];

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
            Value::Array(VecDeque::from([Value::Float(value1), Value::Float(value2)]))
        );

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::json!({}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::Float(value2)]);
    }

    #[tokio::test]
    async fn pop_back_200_array_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1: i64 = Faker.fake();
        let value2: i64 = Faker.fake();
        let value_array = vec![value1, value2];

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
            Value::Array(VecDeque::from([
                Value::Integer(value1),
                Value::Integer(value2)
            ]))
        );

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::json!({}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::Integer(value2)]);
    }

    #[tokio::test]
    async fn pop_back_200_array_float_more() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1 = 1.5;
        let value2 = 1.75;
        let value3 = 2.0;
        let value4 = 2.25;
        let value_array = vec![value1, value2, value3, value4];

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
            Value::Array(VecDeque::from([
                Value::Float(value1),
                Value::Float(value2),
                Value::Float(value3),
                Value::Float(value4)
            ]))
        );

        let pop_back_value = 3;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_back": &pop_back_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body,
            vec![
                Value::Float(value4),
                Value::Float(value3),
                Value::Float(value2)
            ]
        );
    }

    #[tokio::test]
    async fn pop_back_200_array_integer_more() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1: i64 = Faker.fake();
        let value2: i64 = Faker.fake();
        let value3: i64 = Faker.fake();
        let value4: i64 = Faker.fake();
        let value_array = vec![value1, value2, value3, value4];

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
            Value::Array(VecDeque::from([
                Value::Integer(value1),
                Value::Integer(value2),
                Value::Integer(value3),
                Value::Integer(value4)
            ]))
        );

        let pop_back_value = 3;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_back": &pop_back_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body,
            vec![
                Value::Integer(value4),
                Value::Integer(value3),
                Value::Integer(value2)
            ]
        );
    }

    #[tokio::test]
    async fn pop_back_200_array_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1 = Paragraph(2..10).fake::<String>();
        let value2 = Paragraph(2..10).fake::<String>();
        let value_array = vec![value1.clone(), value2.clone()];

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
            Value::Array(VecDeque::from([
                Value::String(value1.clone()),
                Value::String(value2.clone())
            ]))
        );

        let pop_back_value = 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_back": &pop_back_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::String(value2)]);
    }

    #[tokio::test]
    async fn pop_back_200_authentication() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1 = Paragraph(2..10).fake::<String>();
        let value2 = Paragraph(2..10).fake::<String>();
        let value_array = vec![value1.clone(), value2.clone()];

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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([
                Value::String(value1.clone()),
                Value::String(value2.clone())
            ]))
        );

        let pop_back_value = 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({ "pop_back": &pop_back_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::String(value2)]);
    }

    #[tokio::test]
    async fn pop_back_401() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1 = Paragraph(2..10).fake::<String>();
        let value2 = Paragraph(2..10).fake::<String>();
        let value_array = vec![value1.clone(), value2.clone()];

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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([
                Value::String(value1.clone()),
                Value::String(value2.clone())
            ]))
        );

        let pop_back_value = 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_back": &pop_back_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn pop_back_404() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let pop_back_value = 1;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_back": &pop_back_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn pop_back_409() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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

        let pop_back_value = 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_back": &pop_back_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn pop_back_422() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value)]))
        );

        let value = Word().fake::<String>();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-back"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_back": &value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn pop_front_200_array_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1: bool = Faker.fake();
        let value2: bool = Faker.fake();
        let value_array = vec![value1, value2];

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
            Value::Array(VecDeque::from([
                Value::Boolean(value1),
                Value::Boolean(value2)
            ]))
        );

        let pop_front_value = 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_front": &pop_front_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::Boolean(value1)]);
    }

    #[tokio::test]
    async fn pop_front_200_array_boolean_more() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1: bool = Faker.fake();
        let value2: bool = Faker.fake();
        let value3: bool = Faker.fake();
        let value_array = vec![value1, value2, value3];

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
            Value::Array(VecDeque::from([
                Value::Boolean(value1),
                Value::Boolean(value2),
                Value::Boolean(value3)
            ]))
        );

        let pop_front_value = 2;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_front": &pop_front_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::Boolean(value1), Value::Boolean(value2)]);
    }

    #[tokio::test]
    async fn pop_front_200_array_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1 = 1.5;
        let value2 = 1.75;
        let value_array = vec![value1, value2];

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
            Value::Array(VecDeque::from([Value::Float(value1), Value::Float(value2)]))
        );

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::json!({}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::Float(value1)]);
    }

    #[tokio::test]
    async fn pop_front_200_array_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1: i64 = Faker.fake();
        let value2: i64 = Faker.fake();
        let value_array = vec![value1, value2];

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
            Value::Array(VecDeque::from([
                Value::Integer(value1),
                Value::Integer(value2)
            ]))
        );

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(serde_json::json!({}).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::Integer(value1)]);
    }

    #[tokio::test]
    async fn pop_front_200_array_float_more() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1 = 1.5;
        let value2 = 1.75;
        let value3 = 2.0;
        let value4 = 2.25;
        let value_array = vec![value1, value2, value3, value4];

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
            Value::Array(VecDeque::from([
                Value::Float(value1),
                Value::Float(value2),
                Value::Float(value3),
                Value::Float(value4)
            ]))
        );

        let pop_front_value = 3;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_front": &pop_front_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body,
            vec![
                Value::Float(value1),
                Value::Float(value2),
                Value::Float(value3)
            ]
        );
    }

    #[tokio::test]
    async fn pop_front_200_array_integer_more() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1: i64 = Faker.fake();
        let value2: i64 = Faker.fake();
        let value3: i64 = Faker.fake();
        let value4: i64 = Faker.fake();
        let value_array = vec![value1, value2, value3, value4];

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
            Value::Array(VecDeque::from([
                Value::Integer(value1),
                Value::Integer(value2),
                Value::Integer(value3),
                Value::Integer(value4)
            ]))
        );

        let pop_front_value = 3;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_front": &pop_front_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(
            body,
            vec![
                Value::Integer(value1),
                Value::Integer(value2),
                Value::Integer(value3)
            ]
        );
    }

    #[tokio::test]
    async fn pop_front_200_array_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1 = Paragraph(2..10).fake::<String>();
        let value2 = Paragraph(2..10).fake::<String>();
        let value_array = vec![value1.clone(), value2.clone()];

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
            Value::Array(VecDeque::from([
                Value::String(value1.clone()),
                Value::String(value2.clone())
            ]))
        );

        let pop_front_value = 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_front": &pop_front_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::String(value1)]);
    }

    #[tokio::test]
    async fn pop_front_200_authentication() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1 = Paragraph(2..10).fake::<String>();
        let value2 = Paragraph(2..10).fake::<String>();
        let value_array = vec![value1.clone(), value2.clone()];

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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([
                Value::String(value1.clone()),
                Value::String(value2.clone())
            ]))
        );

        let pop_front_value = 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({ "pop_front": &pop_front_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: Vec<Value> = serde_json::from_slice(&body).unwrap();

        assert_eq!(body, vec![Value::String(value1)]);
    }

    #[tokio::test]
    async fn pop_front_401() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value1 = Paragraph(2..10).fake::<String>();
        let value2 = Paragraph(2..10).fake::<String>();
        let value_array = vec![value1.clone(), value2.clone()];

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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([
                Value::String(value1.clone()),
                Value::String(value2.clone())
            ]))
        );

        let pop_front_value = 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_front": &pop_front_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn pop_front_404() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let pop_front_value = 1;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_front": &pop_front_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn pop_front_409() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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

        let pop_front_value = 1;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_front": &pop_front_value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn pop_front_422() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value)]))
        );

        let value = Word().fake::<String>();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/pop-front"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "pop_front": &value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn prepend_200_array_array_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Boolean(value)
            ]))]))
        );

        let prepend_value: bool = Faker.fake();
        let prepend_value_array = vec![vec![prepend_value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::Array(VecDeque::from([Value::Boolean(prepend_value)])),
                Value::Array(VecDeque::from([Value::Boolean(value)]))
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_array_array_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 1.5;
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Float(value)
            ]))]))
        );

        let prepend_value = 1.75;
        let prepend_value_array = vec![vec![prepend_value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::Array(VecDeque::from([Value::Float(prepend_value)])),
                Value::Array(VecDeque::from([Value::Float(value)]))
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_array_array_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Integer(value)
            ]))]))
        );

        let prepend_value: i64 = Faker.fake();
        let prepend_value_array = vec![vec![prepend_value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::Array(VecDeque::from([Value::Integer(prepend_value)])),
                Value::Array(VecDeque::from([Value::Integer(value)]))
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_array_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Boolean(value)]))
        );

        let prepend_value: bool = Faker.fake();
        let prepend_value_array = vec![prepend_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::Boolean(prepend_value),
                Value::Boolean(value)
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_array_boolean_and_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Boolean(value)]))
        );

        let prepend_value = 1.5;
        let prepend_value_array = vec![prepend_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::Float(prepend_value),
                Value::Boolean(value)
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_array_boolean_and_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Boolean(value)]))
        );

        let prepend_value: i64 = Faker.fake();
        let prepend_value_array = vec![prepend_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::Integer(prepend_value),
                Value::Boolean(value)
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_array_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 1.5;
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Float(value)]))
        );

        let prepend_value = 1.75;
        let prepend_value_array = vec![prepend_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::Float(prepend_value),
                Value::Float(value)
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_array_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Integer(value)]))
        );

        let prepend_value: i64 = Faker.fake();
        let prepend_value_array = vec![prepend_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::Integer(prepend_value),
                Value::Integer(value)
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_array_float_and_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 1.5;
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Float(value)]))
        );

        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::String(prepend_value),
                Value::Float(value)
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_array_integer_and_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Integer(value)]))
        );

        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::String(prepend_value),
                Value::Integer(value)
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_array_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value.clone())]))
        );

        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::String(prepend_value),
                Value::String(value)
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_array_string_and_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value.clone())]))
        );

        let prepend_value: bool = Faker.fake();
        let prepend_value_array = vec![prepend_value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::Boolean(prepend_value),
                Value::String(value)
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_200_authentication() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value.clone())]))
        );

        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
            Value::Array(VecDeque::from([
                Value::String(prepend_value),
                Value::String(value)
            ]))
        );
    }

    #[tokio::test]
    async fn prepend_401() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value)]))
        );

        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let prepend_value = Paragraph(2..10).fake::<String>();
        let prepend_value_array = vec![prepend_value.clone()];

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}/prepend"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}/prepend"))
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
    async fn read_200() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri(format!("/values/{key}"))
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Boolean(value)
            ]))]))
        );

        let value: bool = Faker.fake();
        let value_array = vec![vec![value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value_array }).to_string(),
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Boolean(value)
            ]))]))
        );
    }

    #[tokio::test]
    async fn update_200_array_array_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 1.5;
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Float(value)
            ]))]))
        );

        let value = 1.5;
        let value_array = vec![vec![value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value_array }).to_string(),
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Float(value)
            ]))]))
        );
    }

    #[tokio::test]
    async fn update_200_array_array_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Integer(value)
            ]))]))
        );

        let value: i64 = Faker.fake();
        let value_array = vec![vec![value]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value_array }).to_string(),
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::Integer(value)
            ]))]))
        );
    }

    #[tokio::test]
    async fn update_200_array_array_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::String(value)
            ]))]))
        );

        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![vec![value.clone()]];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value_array }).to_string(),
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
            Value::Array(VecDeque::from([Value::Array(VecDeque::from([
                Value::String(value)
            ]))]))
        );
    }

    #[tokio::test]
    async fn update_200_array_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Boolean(value)]))
        );

        let value: bool = Faker.fake();
        let value_array = vec![value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value_array }).to_string(),
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
            Value::Array(VecDeque::from([Value::Boolean(value)]))
        );
    }

    #[tokio::test]
    async fn update_200_array_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 1.5;
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Float(value)]))
        );

        let value = 1.5;
        let value_array = vec![value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value_array }).to_string(),
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
            Value::Array(VecDeque::from([Value::Float(value)]))
        );
    }

    #[tokio::test]
    async fn update_200_array_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::Integer(value)]))
        );

        let value: i64 = Faker.fake();
        let value_array = vec![value];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value_array }).to_string(),
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
            Value::Array(VecDeque::from([Value::Integer(value)]))
        );
    }

    #[tokio::test]
    async fn update_200_array_string() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
        assert_eq!(
            body.value,
            Value::Array(VecDeque::from([Value::String(value)]))
        );

        let value = Paragraph(2..10).fake::<String>();
        let value_array = vec![value.clone()];

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value_array }).to_string(),
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
            Value::Array(VecDeque::from([Value::String(value)]))
        );
    }

    #[tokio::test]
    async fn update_200_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
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
    async fn update_200_boolean_to_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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

        let value = 1.5;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Float(value));
    }

    #[tokio::test]
    async fn update_200_boolean_to_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
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
    async fn update_200_float() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 1.5;

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
        assert_eq!(body.value, Value::Float(value));

        let value = 1.5;

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: ValueResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.key, key);
        assert_eq!(body.value, Value::Float(value));
    }

    #[tokio::test]
    async fn update_200_integer() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
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
    async fn update_200_float_to_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;
        let cloned_router = router.clone();

        let key = Word().fake::<String>();
        let value = 1.5;

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
        assert_eq!(body.value, Value::Float(value));

        let value = Faker.fake();

        let response = cloned_router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
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
    async fn update_200_integer_to_boolean() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
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
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
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
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
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
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn update_404() {
        let db_config = DbConfig {
            enable_security_api_keys: false,
            ..Default::default()
        };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let key = Word().fake::<String>();
        let value = Paragraph(2..10).fake::<String>();

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::PUT)
                    .uri(format!("/values/{key}"))
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::json!({ "value": &value }).to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
