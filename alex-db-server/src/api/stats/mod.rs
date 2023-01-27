use crate::{access::Access, error::AppError};
use alex_db_lib::db::Db;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use std::sync::Arc;

mod test;

#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "/stats",
    responses(
        (status = 200, description = "Stats read.", body = StatRecord),
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
) -> Result<impl IntoResponse, AppError> {
    if !access.granted() {
        return Err(AppError::Unauthorized);
    }

    let stats = db.get_stats()?;

    Ok((StatusCode::OK, Json(stats)).into_response())
}
