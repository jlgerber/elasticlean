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

/// Responsible for providing basic crud over indices
#[derive(Debug)]
pub struct Elasticrud {
    host: String,
    port: u16,
}


impl Elasticrud {
    /// New up an instance of Elasticrud given a host and port number
    pub fn new<I>(host: I, port: u16) -> Elasticrud
    where
        I: Into<String>
    {
        Elasticrud {
            host: host.into(),
            port,
        }
    }

    /// retrieve a list of indices
    pub fn get_raw_indices(&self) -> Result<Vec<RawIndex>, EcError> {
        let route = self.get_route("_cat/indices?format=json");
        debug!("Elasticrud.get - route {}", route);

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
    /// let results = ec.get::<MyIndexData>(indices)?;
    /// ```
    pub fn get<I>(&self, indices: &Vec<Index>) -> Result<Vec<I>, EcError>
    where for<'de> I: serde::Deserialize<'de>
    {
        // build a comma separated string of indexes
        let indices = indices.into_iter()
            .map(|i| format!("{}",i))
            .collect::<Vec<String>>()
            .join(",");

        // build a search route
        let route = self.get_route(format!("{}/_search", indices).as_str());
        info!("get_indices route {}", route);
        let body: EVal<I> = reqwest::get(&route)
                                .map_err(|e| EcError::ReqwestGetError(format!("{}",e)))?
                                .json()
                                .map_err(|e| EcError::ReqwestJsonError(format!("{}",e)))?;

        // extract a vector of the supplied type from the json returned by
        // the query and return the results.
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
        debug!("Elasticrud.delete_indices - route {}", route);

        let client = reqwest::Client::new();
        client.delete(&route)
            .send()
            .map_err(|e| EcError::ReqwestGetError(format!("{}",e)))

    }

    // build a uri given the resource
    fn get_route(&self, resource: &str) -> String {
        format!("http://{}:{}/{}", self.host, self.port, resource)
    }

}

