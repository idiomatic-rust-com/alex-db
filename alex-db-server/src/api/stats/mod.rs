use crate::error::AppError;
use alex_db_lib::db::Db;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "/stats",
    responses(
        (status = 200, description = "List of the stats", body = StatRecord),
    )
)]
pub async fn list(State(db): State<Arc<Db>>) -> Result<impl IntoResponse, AppError> {
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
            saved_writes_threshold: 8,
        };
        let router = app::get_app(config).await.unwrap();

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
}
