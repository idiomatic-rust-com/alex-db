use crate::error::ResponseError;
use alex_db_lib::{
    db::Db,
    stat_record::StatRecord,
    value_record::{Value, ValuePost, ValuePut, ValueResponse},
};
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{delete, get},
    Router,
};
use std::{sync::Arc, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod stats;
mod values;

pub async fn router(db: Arc<Db>) -> Router {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            stats::list,
            values::create,
            values::delete,
            values::list,
            values::read,
            values::update,
        ),
        components(
            schemas(
                ResponseError,
                StatRecord,
                Value,
                ValuePost,
                ValuePut,
                ValueResponse,
            )
        ),
        tags(
            (name = "stats", description = "Stats API"),
            (name = "values", description = "Values management API"),
        )
    )]
    struct ApiDoc;

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route("/stats", get(stats::list))
        .route("/values", get(values::list).post(values::create))
        .route(
            "/values/:key",
            delete(values::delete).get(values::read).put(values::update),
        )
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
