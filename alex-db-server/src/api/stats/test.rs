#[cfg(test)]
mod tests {
    use crate::{app, config::Config};
    use alex_db_lib::{config::Config as DbConfig, stat_record::StatRecord};
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn list_200() {
        let db_config = DbConfig { enable_security_api_keys: false, ..Default::default() };
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/stats")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: StatRecord = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.reads, 0);
        assert_eq!(body.requests, 0);
        assert_eq!(body.saved_writes, 0);
        assert_eq!(body.writes, 0);
    }

    #[tokio::test]
    async fn list_200_authentication() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/stats")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .header("X-Auth-Token".to_string(), app.api_key.unwrap().to_string())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: StatRecord = serde_json::from_slice(&body).unwrap();

        assert_eq!(body.reads, 0);
        assert_eq!(body.requests, 0);
        assert_eq!(body.saved_writes, 0);
        assert_eq!(body.writes, 0);
    }

    #[tokio::test]
    async fn list_401() {
        let db_config = DbConfig::default();
        let config = Config::new(db_config, 10240);
        let app = app::get_app(config).await.unwrap();
        let router = app.router;

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/stats")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
