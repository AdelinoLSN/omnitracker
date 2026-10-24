use anyhow::{Context, Ok, Result};
use axum::serve;
use tokio::net::TcpListener;

use crate::{config::Config, routes::create_router};

pub async fn run(config: Config) -> Result<()> {
    let listener = TcpListener::bind((config.host.as_str(), config.port))
        .await
        .context("Failed to bind server addres")?;

    println!("Runnig on {}:{}", config.host, config.port);

    serve(listener, create_router())
        .await
        .context("Failed to start HTTP server")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fails_when_binding_to_invalid_address() {
        let invalid_config = Config {
            host: "999.999.999.999".to_string(),
            port: 80,
        };

        let result = run(invalid_config).await;

        assert!(result.is_err());
        let err_message = result.unwrap_err().to_string();
        assert!(err_message.contains("Failed to bind server addres"));
    }

    #[tokio::test]
    async fn fails_when_port_is_already_in_use() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();

        let config = Config {
            host: "127.0.0.1".to_string(),
            port,
        };

        let result = run(config).await;

        assert!(result.is_err());
        let err_message = result.unwrap_err().to_string();
        assert!(err_message.contains("Failed to bind server addres"));
        
        drop(listener);
    }
}
