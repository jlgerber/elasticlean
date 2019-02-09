//! # rawindex.rs
//!
//! Herein we define a RawIndex struct which models metadata
//! returned by elasticsearch when querying the state of an index

/// The RawIndex struct reflects the Index data structure
/// in returned by Elasticsearch.
/// It is designed to be deserialized via serde_json
#[derive(Deserialize, Debug)]
pub struct RawIndex {
    pub health: String,
    pub status: String,
    pub index: String,
    pub pri: String,
    pub rep: String,
    #[serde(rename = "store.size")]
    pub store_size: String,
    #[serde(rename = "pri.store.size")]
    pub pri_store_size: String,
}
