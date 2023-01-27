use crate::error::ResponseError;
use alex_db_lib::{
    db::Db,
    stat_record::StatRecord,
    value_record::{
        Value, ValueAppend, ValueDecrement, ValueIncrement, ValuePopBack, ValuePopFront, ValuePost,
        ValuePrepend, ValuePut, ValueResponse,
    },
};
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    routing::{delete, get, put},
    Router,
};
use std::{sync::Arc, time::Duration};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

mod stats;
mod values;

pub async fn router(db: Arc<Db>) -> Router {
    #[derive(OpenApi)]
    #[openapi(
        components(
            schemas(
                ResponseError,
                StatRecord,
                Value,
                ValueAppend,
                ValueDecrement,
                ValueIncrement,
                ValuePopBack,
                ValuePopFront,
                ValuePost,
                ValuePrepend,
                ValuePut,
                ValueResponse,
            )
        ),
        modifiers(&SecurityAddon),
        paths(
            stats::list,
            values::append,
            values::create,
            values::decrement,
            values::delete,
            values::increment,
            values::list,
            values::pop_back,
            values::pop_front,
            values::prepend,
            values::read,
            values::update,
        ),
        tags(
            (name = "stats", description = "Stats API."),
            (name = "values", description = "Values management API."),
        )
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "api_key",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-Auth-Token"))),
                )
            }
        }
    }

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route("/stats", get(stats::list))
        .route("/values", get(values::list).post(values::create))
        .route(
            "/values/:key",
            delete(values::delete).get(values::read).put(values::update),
        )
        .route("/values/:key/append", put(values::append))
        .route("/values/:key/decrement", put(values::decrement))
        .route("/values/:key/increment", put(values::increment))
        .route("/values/:key/pop-back", put(values::pop_back))
        .route("/values/:key/pop-front", put(values::pop_front))
        .route("/values/:key/prepend", put(values::prepend))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(db)
}
