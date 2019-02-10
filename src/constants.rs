//! # constants.rs
//!
//! Constants used in the program
//!

/// The minimum number of days if not specified
pub const MIN_DAYS: i32 = 60;

/// The environment variable name to specify the elasticsearch host.
pub(crate) const HOSTVAR: &'static str = "ELASTICLEAN_HOST";

/// The environment variable name to specify the elasticsearch port.
pub(crate) const PORTVAR: &'static str = "ELASTICLEAN_PORT";

pub(crate) const MIN_DAYS_VAR: &'static str = "ELASTICLEAN_MIN_DAYS";