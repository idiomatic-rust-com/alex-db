use crate::{api, config::Config, Result};
use alex_db_lib::db::Db;
use axum::Router;
use std::sync::Arc;
use tokio::{
    task,
    time::{sleep, Duration},
};
use tracing::error;

pub async fn get_app(config: Config) -> Result<Router> {
    let mut db = Db::new(config.data_dir, config.saved_writes_threshold);
    db.restore()?;
    let db = Arc::new(db);
    let cloned_db = db.clone();

    let app = api::router(db).await;

    task::spawn(async move {
        loop {
            let res = cloned_db.save();

            if let Err(e) = res {
                error!("Error: {:?}", e);
            }

            sleep(Duration::from_secs(10)).await;
        }
    });

    Ok(app)
}
