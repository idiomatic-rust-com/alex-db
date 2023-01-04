use std::{error::Error, net::SocketAddr};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

mod api;
mod app;
mod error;

pub async fn run() -> Result<()> {
    let app = app::get_app().await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
