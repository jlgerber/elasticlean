#[derive(Debug)]
pub struct Elasticleaner {
    host: String,
    port: u16,
}

impl Elasticleaner {
    pub fn new<I>(host: I, port: u16) -> Elasticleaner
    where
        I: Into<String>
    {
        Elasticleaner {
            host: host.into(),
            port,
        }
    }
}
use serde_json::{Value, Error};
use failure;
use reqwest;

#[derive(Deserialize, Debug)]
pub struct RetVal {
    pub index: String,
}

impl Elasticleaner {
    /// retrieve a list of indices
    pub fn get_indices(&self) -> Result<Vec<RetVal>, failure::Error> {
        let route = self.get_route("_cat/indices?format=json");
        let body: Vec<RetVal> = reqwest::get(&route)?
                    .json()?;
        Ok(body)
    }

    fn get_route(&self, resource: &str) -> String {
        format!("http://{}:{}/{}", self.host, self.port, resource)
    }
}

