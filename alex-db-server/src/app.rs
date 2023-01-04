use crate::{api, Result};
use alex_db_lib::db::Db;
use axum::Router;
use std::sync::Arc;

pub async fn get_app() -> Result<Router> {
    let db = Arc::new(Db::new(None));

    let app = api::router(db).await;

    Ok(app)
}
