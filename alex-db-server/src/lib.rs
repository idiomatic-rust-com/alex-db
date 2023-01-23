use clap::Parser;
use std::{error::Error, net::SocketAddr};
use tokio::{
    task,
    time::{sleep, Duration},
};
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

mod access;
mod api;
mod app;
mod config;
mod error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Data directory
    #[arg(short, long)]
    pub data_dir: Option<String>,

    /// Enable API Key endpoint protection
    #[arg(short, long)]
    pub enable_security_api_keys: Option<bool>,

    /// Port
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Database save triggered after write operations threshold
    #[arg(long)]
    pub save_triggered_by_threshold: Option<u16>,

    /// Database save triggered after time in ms
    #[arg(long)]
    pub save_triggered_after_ms: Option<i64>,

    /// Sleep time between database gc in ms
    #[arg(long)]
    pub sleep_time_between_gc_ms: Option<u64>,

    /// Sleep time between database saves in ms
    #[arg(long)]
    pub sleep_time_between_saves_ms: Option<u64>,
}

pub async fn run() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "alex_db_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args = Args::parse();
    let config = config::load(args)?;

    let app = app::get_app(config.clone()).await?;

    let db_for_deleting = app.db.clone();
    task::spawn(async move {
        loop {
            let res = db_for_deleting.gc();

            if let Err(e) = res {
                error!("Error: {:?}", e);
            }

            sleep(Duration::from_millis(
                config.db_config.sleep_time_between_gc_ms,
            ))
            .await;
        }
    });

    let db_for_saving = app.db;
    task::spawn(async move {
        loop {
            let res = db_for_saving.save();

            if let Err(e) = res {
                error!("Error: {:?}", e);
            }

            sleep(Duration::from_millis(
                config.db_config.sleep_time_between_saves_ms,
            ))
            .await;
        }
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
