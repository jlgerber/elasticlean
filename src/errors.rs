//! # errors.rs
//!
//! Custom Errors for elasticlean
//!
#[derive(Fail, Debug,PartialEq,Eq, PartialOrd, Ord)]
pub enum EcError {
    /// Parsing error
    #[fail(display = "failed to parse: {}", _0)]
    ParseError(String),
    /// General Errors originating in the Reqwest module
    #[fail(display = "reqwest get error: {}", _0)]
    ReqwestGetError(String),
    /// Json error originating in the Reqwest module
    #[fail(display = "reqwest json error. unable to deserialize to json: {}", _0)]
    ReqwestJsonError(String),
    /// Error raised when a feature is not implemented yet
    #[fail(display = "NotImplemented")]
    NotImplemented,
    /// Failure to find look up the specified variable in the environment
    #[fail(display = "{} not found in environment", _0)]
    EnvVarError(String),
}