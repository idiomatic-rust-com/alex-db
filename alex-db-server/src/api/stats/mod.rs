use crate::{access::Access, error::AppError};
use alex_db_lib::db::Db;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "/stats",
    responses(
        (status = 200, description = "List of the stats", body = StatRecord),
        (status = 401, description = "Unauthorized", body = ResponseError),
    )
)]
pub async fn list(
    access: Access,
    State(db): State<Arc<Db>>,
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    let stats = db.get_stats()?;

    Ok((StatusCode::OK, Json(stats)).into_response())
}

#[cfg(test)]
mod tests {
    use crate::{app, config::Config};
    use alex_db_lib::stat_record::StatRecord;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn list_200() {
        let config = Config {
            data_dir: None,
            port: 8080,
            saved_writes_sleep: 10000,
            saved_writes_threshold: 8,
            saved_writes_trigger_after: 60000,
            security_api_keys: false,
        };
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
        let config = Config {
            data_dir: None,
            port: 8080,
            saved_writes_sleep: 10000,
            saved_writes_threshold: 8,
            saved_writes_trigger_after: 60000,
            security_api_keys: true,
        };
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
        let config = Config {
            data_dir: None,
            port: 8080,
            saved_writes_sleep: 10000,
            saved_writes_threshold: 8,
            saved_writes_trigger_after: 60000,
            security_api_keys: true,
        };
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
