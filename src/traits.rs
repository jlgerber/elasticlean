use serde::de::DeserializeOwned;
use std::fmt::Display;

/// Trait which must be implemented by an index struct
/// in order to work with Elasticlean. This simple trait defines
/// the base name of the index as a constant.
pub trait ElasticIndex: DeserializeOwned + Display {
    const NAME: &'static str; // the index name
}