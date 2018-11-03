
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
