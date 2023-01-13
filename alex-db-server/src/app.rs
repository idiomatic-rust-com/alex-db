use crate::{api, config::Config, Result};
use alex_db_lib::db::Db;
use axum::Router;
use std::sync::Arc;
use tokio::{
    task,
    time::{sleep, Duration},
};
use tracing::{error, info};
use uuid::Uuid;

pub struct App {
    pub api_key: Option<Uuid>,
    pub router: Router,
}

pub async fn get_app(config: Config) -> Result<App> {
    let restricted_access = config.security_api_keys;
    let mut db = Db::new(
        config.data_dir,
        restricted_access,
        config.saved_writes_threshold,
    );
    db.restore()?;
    let mut api_key = None;

    if config.security_api_keys {
        api_key = db.api_key_init()?;
        if api_key.is_some() {
            info!("initial api key created: {:?}", api_key);
        }
    }

    let db = Arc::new(db);
    let cloned_db = db.clone();

    let router = api::router(db).await;

    task::spawn(async move {
        loop {
            let res = cloned_db.save();

            if let Err(e) = res {
                error!("Error: {:?}", e);
            }

            sleep(Duration::from_millis(config.saved_writes_sleep)).await;
        }
    });

    let app = App { api_key, router };

    Ok(app)
}
