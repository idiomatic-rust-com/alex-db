use crate::{api, config::Config, Result};
use alex_db_lib::db::Db;
use axum::Router;
use std::sync::Arc;

pub async fn get_app(config: Config) -> Result<Router> {
    let mut db = Db::new(config.data_dir);
    db.restore();
    let db = Arc::new(db);

    let app = api::router(db).await;

    Ok(app)
}
