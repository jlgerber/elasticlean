use crate::constants::{HOSTVAR, PORTVAR};
use std::env::var;
use crate::errors::EcError;

/// Store configurable data, like the elasticsearch host name
/// and port. The struct provides functions to generate a Config
/// from environment variables.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn new<I, J>(host: I, port: J) -> Config
    where
        I: Into<String>,
        J: Into<String>
    {
        Config {
            host: host.into(),
            port: port.into().parse::<u16>().unwrap()
        }
    }
    /// New up a Config from environment variables. Return a result.
    pub fn from_env() -> Result<Config, EcError> {
        let host = var(HOSTVAR).map_err(|_| { EcError::EnvVarError(HOSTVAR.to_string())} )?;
        let port = var(PORTVAR).map_err(|_| {EcError::EnvVarError(PORTVAR.to_string())})?;
        Ok(Config::new(host, port))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn newconfig() {
        let nc = Config::new("foo", "16");
        let expect = Config {
            host: "foo".to_string(),
            port: 16,
        };

        assert_eq!(nc, expect);
    }
}