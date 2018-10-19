use reqwest;
use errors::EcError;

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

#[derive(Deserialize, Debug)]
pub struct RetVal {
    pub index: String,
}

impl Elasticleaner {
    /// retrieve a list of indices
    pub fn get_indices(&self) -> Result<Vec<RetVal>, EcError> {
        let route = self.get_route("_cat/indices?format=json");
        debug!("Elasticleaner.get_indices - route {}", route);

        let body: Vec<RetVal> = reqwest::get(&route)
                                .map_err(|e| EcError::ReqwestGetError(format!("{}",e)))?
                                .json()
                                .map_err(|e| EcError::ReqwestJsonError(format!("{}",e)))?;
        Ok(body)
    }
        /// retrieve a list of indices
    pub fn delete_indices(&self, indices: String) -> Result<reqwest::Response, EcError> {
        let route = self.get_route(indices.as_str());
        debug!("Elasticleaner.delete_indices - route {}", route);

        let client = reqwest::Client::new();
        client.delete(&route)
            .send()
            .map_err(|e| EcError::ReqwestGetError(format!("{}",e)))

       // Ok(())
    }

    fn get_route(&self, resource: &str) -> String {
        format!("http://{}:{}/{}", self.host, self.port, resource)
    }

}

