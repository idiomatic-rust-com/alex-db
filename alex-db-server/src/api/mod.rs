use alex_db_lib::db::Db;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::{sync::Arc, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

mod values;

pub async fn router(db: Arc<Db>) -> Router {
    Router::new()
        .route("/values", post(values::post))
        .route("/values/:key", get(values::get))
        //.route("/values", get(values::list).post(values::post))
        //.route("/values/:key", delete(values::delete).get(values::get).put(values::put))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(db)
}
