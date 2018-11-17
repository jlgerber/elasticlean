use serde::de::DeserializeOwned;
use std::fmt::Display;

pub trait ElasticIndex: DeserializeOwned + Display {
    const NAME: &'static str; // the index name
}