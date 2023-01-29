#![forbid(unsafe_code)]

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    alex_db_client::run().await?;

    Ok(())
}
