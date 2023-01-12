use clap::Parser;
use std::{error::Error, net::SocketAddr};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

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

    /// Port
    #[arg(short, long)]
    pub port: Option<u16>,

    /// Threshold
    #[arg(short, long)]
    pub saved_writes_threshold: Option<u16>,
}

pub async fn run() -> Result<()> {
    let args = Args::parse();
    let config = config::load(args)?;

    let app = app::get_app(config.clone()).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
