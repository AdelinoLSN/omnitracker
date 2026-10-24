use anyhow::{Context, bail};
use std::env;

#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let host = env::var("HOST").context("HOST environment variable must be set")?;

        let port: u16 = env::var("PORT")
            .context("PORT environment variable must be set")?
            .parse()
            .context("PORT must be a valid port number")?;

        if port == 0 {
            bail!("PORT must be greater than 0");
        }

        Ok(Self { host, port })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn clear_env() {
        unsafe {
            env::remove_var("HOST");
            env::remove_var("PORT");
        }
    }

    fn set_env(host: &str, port: &str) {
        unsafe {
            env::set_var("HOST", host);
            env::set_var("PORT", port);
        }
    }

    #[test]
    fn from_env_success() {
        clear_env();
        set_env("127.0.0.1", "3000");

        let config = Config::from_env().unwrap();

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);

        clear_env();
    }

    #[test]
    fn fails_when_host_is_missing() {
        clear_env();
        unsafe { env::set_var("PORT", "3000") };

        let err = Config::from_env().unwrap_err();
        assert!(err.to_string().contains("HOST environment variable must be set"));

        clear_env();
    }

    #[test]
    fn fails_when_port_is_missing() {
        clear_env();
        unsafe { env::set_var("HOST", "127.0.0.1") };

        let err = Config::from_env().unwrap_err();
        assert!(err.to_string().contains("PORT environment variable must be set"));

        clear_env();
    }

    #[test]
    fn fails_when_port_is_not_a_number() {
        clear_env();
        set_env("127.0.0.1", "abc");

        let err = Config::from_env().unwrap_err();
        assert!(err.to_string().contains("PORT must be a valid port number"));

        clear_env();
    }

    #[test]
    fn fails_when_port_is_zero() {
        clear_env();
        set_env("127.0.0.1", "0");

        let err = Config::from_env().unwrap_err();
        assert!(err.to_string().contains("PORT must be greater than 0"));

        clear_env();
    }
}
