use reqwest;
use errors::EcError;
use rawindex::RawIndex;
use index::Index;
use serde_json;
use serde;
use serde::Deserialize;

/// The outer map returned by elasticsearch _search results
#[derive(Deserialize, Debug)]
pub struct EVal<I> {
    pub hits: Hits<I>,
}

/// The value of EVal.hits is a
#[derive(Deserialize, Debug)]
pub struct Hits<I> {
    pub total: i32,
    pub max_score: f32,
    hits: Vec<EWrap<I>>
}

#[derive(Deserialize, Debug)]
pub struct EWrap<I> {
    #[serde(rename = "_index")]
    pub index: String,
    #[serde(rename = "_type")]
    pub etype: String,
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_score")]
    pub score: f32,
    #[serde(rename = "_source")]
    pub source: I,
}

#[derive(Debug)]
pub struct Elasticleaner {
    host: String,
    port: u16,
}

impl Elasticleaner {
    /// New up an instance of Elasticleaner given a host and port number
    pub fn new<I>(host: I, port: u16) -> Elasticleaner
    where
        I: Into<String>
    {
        Elasticleaner {
            host: host.into(),
            port,
        }
    }

    /// retrieve a list of indices
    pub fn get_indices(&self) -> Result<Vec<RawIndex>, EcError> {
        let route = self.get_route("_cat/indices?format=json");
        debug!("Elasticleaner.get_indices - route {}", route);

        let body: Vec<RawIndex> = reqwest::get(&route)
                                .map_err(|e| EcError::ReqwestGetError(format!("{}",e)))?
                                .json()
                                .map_err(|e| EcError::ReqwestJsonError(format!("{}",e)))?;
        Ok(body)
    }

    /// Retrieve data of a parameterized type
    ///
    /// # usage
    /// ```
    /// let indices = "foobar-2018.10.02";
    /// let results = ec.get_data::<MyIndexData>(indices)?;
    /// ```
    pub fn get_data<I>(&self, indices: &Vec<Index>) -> Result<Vec<I>, EcError>
    where for<'de> I: serde::Deserialize<'de>
    {
        let indices = indices.into_iter()
            .map(|i| format!("{}",i))
            .collect::<Vec<String>>()
            .join(",");

        let route = self.get_route(format!("{}/_search", indices).as_str());
        info!("get_data route {}", route);
        let body: EVal<I> = reqwest::get(&route)
                                .map_err(|e| EcError::ReqwestGetError(format!("{}",e)))?
                                .json()
                                .map_err(|e| EcError::ReqwestJsonError(format!("{}",e)))?;

        // oh boy this is bad
        let res = body.hits.hits.into_iter().map(|x| x.source).collect::<Vec<I>>();
        Ok(res)
    }

    /// Delete one or more indices, provided as a string
    pub fn delete_indices(&self, indices: &Vec<Index>) -> Result<reqwest::Response, EcError> {
        // generate a String
        let idxs = indices.into_iter()
        .map(|i| format!("{}",i))
        .collect::<Vec<String>>()
        .join(",");
        // get route from idxs
        let route = self.get_route(idxs.as_str());
        debug!("Elasticleaner.delete_indices - route {}", route);

        let client = reqwest::Client::new();
        client.delete(&route)
            .send()
            .map_err(|e| EcError::ReqwestGetError(format!("{}",e)))

    }


    fn get_route(&self, resource: &str) -> String {
        format!("http://{}:{}/{}", self.host, self.port, resource)
    }

}

