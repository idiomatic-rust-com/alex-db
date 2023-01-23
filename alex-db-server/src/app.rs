use crate::{api, config::Config, Result};
use alex_db_lib::db::Db;
use axum::Router;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

pub struct App {
    pub api_key: Option<Uuid>,
    pub db: Arc<Db>,
    pub router: Router,
}

pub async fn get_app(config: Config) -> Result<App> {
    let mut db = Db::new(config.db_config.clone());
    db.restore()?;
    let mut api_key = None;

    if config.db_config.enable_security_api_keys {
        api_key = db.api_key_init()?;
        if api_key.is_some() {
            info!("initial api key created: {:?}", api_key);
        }
    }

    let db = Arc::new(db);

    let router = api::router(db.clone()).await;

    let app = App {
        api_key,
        db,
        router,
    };

    Ok(app)
}
