mod config;
mod routes;
mod server;

use anyhow::Result;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let config = Config::from_env()?;

    server::run(config).await
}
