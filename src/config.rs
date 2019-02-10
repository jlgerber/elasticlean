//! config.rs
//!
//! Configuration data management
use crate::{
    constants::{HOSTVAR, PORTVAR, MIN_DAYS_VAR},
    errors::EcError,
 };
use std::env::var;

/// Store configurable data, like the elasticsearch host name
/// and port. The struct provides functions to generate a Config
/// from environment variables.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub min_days: u32,
}

impl Config {
    /// New up a Config given a host and port.
    ///
    /// # Parameters
    ///
    /// * `host` - The host url as a &str or String
    /// * `port` - The port number as a &str or String
    ///
    /// # Returns
    ///
    /// * `Config` instance
    pub fn new<I, J, K>(host: I, port: J, min_days: K) -> Config
    where
        I: Into<String>,
        J: Into<String>,
        K: Into<String>,
    {
        Config {
            host: host.into(),
            port: port.into().parse::<u16>().unwrap(),
            min_days: min_days.into().parse::<u32>().unwrap(),
        }
    }

    /// New up a Config, grabbing the host and port from environment variables.
    ///
    /// # Environment Variables (defined in constants)
    ///
    /// * `ELASTICLEAN_HOST` - Name of the host
    /// * `ELASTICLEAN_PORT` - Port number
    ///
    /// # Returns
    ///
    /// * `Config` instance if successful
    /// * `ExError` instance if failure
    pub fn from_env() -> Result<Config, EcError> {
        let host = var(HOSTVAR).map_err(|_| { EcError::EnvVarError(HOSTVAR.to_string())} )?;
        let port = var(PORTVAR).map_err(|_| {EcError::EnvVarError(PORTVAR.to_string())})?;
        let min_days = var(MIN_DAYS_VAR).map_err(|_| {EcError::EnvVarError(MIN_DAYS_VAR.to_string())})?;
        Ok(Config::new(host, port, min_days))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_new_up_config() {
        let nc = Config::new("foo", "16", "5");
        let expect = Config {
            host: "foo".to_string(),
            port: 16,
            min_days: 5,
        };

        assert_eq!(nc, expect);
    }
}