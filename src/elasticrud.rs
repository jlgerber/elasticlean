//! # elasticrud.rs
//!
//! Communication with Elasticsearch via REST.
//!
//! This file is an implementation detail. The classes
//! are not exposed as public api.
//!
use crate::{
    errors::EcError,
    index::Index,
    rawindex::RawIndex,
    traits::ElasticIndex,
};
use reqwest;

/// The outer map returned by elasticsearch _search results
#[derive(Deserialize, Debug)]
pub(crate) struct EsSearchRoot<I> {
    pub hits: Hits<I>,
}

/// The value of EsSearchRoot.hits
#[derive(Deserialize, Debug)]
pub(crate) struct Hits<I> {
    pub total: i32,
    pub max_score: f32,
    hits: Vec<EsSearchMeta<I>>
}

// metadata wrapper containing
#[derive(Deserialize, Debug)]
pub(crate) struct EsSearchMeta<I> {
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
pub struct Elasticrud<'a> {
    host: &'a str,
    port: u16,
}


impl<'a> Elasticrud<'a> {
    /// New up an instance of Elasticrud given a host and port number
    ///
    /// # Parameters
    ///
    /// * `host` - The host url
    /// * `port` - The port number
    ///
    /// # Returns
    ///
    /// * `Elasticrud` instance
    pub fn new(host: &'a str, port: u16) -> Elasticrud<'a>

    {
        Elasticrud {
            host: host,//.into(),
            port,
        }
    }

    /// Retrieve a list of indices from elasticsearch
    ///
    /// # Parameters
    ///
    /// None
    ///
    /// # Returns
    ///
    /// * `Vector` of `RawIndex` instances if successful
    /// * `EcError` if unsuccesful
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
    /// Note: this method assumes that the indices are deserializable
    /// into the I type when retrieved from elasticsearch. If this is not
    /// the case, an error will be returned at runtime.
    ///
    /// # Parameters
    ///
    /// * `indices` - References to a `Vector` of `Index`instances
    ///
    /// # Returns
    ///
    /// * `Vector` of `ElasticIndex` implementers if successful
    /// * `EcError`` instance if unsuccessful
    ///
    /// # Usage
    ///
    /// ```rust,ignore
    /// let indices = vec![Index::from_str("foobar-2018.10.02")?];
    /// let results = ec.get::<MyIndexData>(indices)?;
    /// ```
    pub fn get<I>(&self, indices: &Vec<Index>) -> Result<Vec<I>, EcError>
    where
        //for<'de> I: serde::Deserialize<'de>
        I: ElasticIndex
    {
        // build a comma separated string of indexes
        let indices = indices.into_iter()
            .map(|i| format!("{}",i))
            .collect::<Vec<String>>()
            .join(",");

        // build a search route
        let route = self.get_route(format!("{}/_search", indices).as_str());
        info!("get_indices route {}", route);
        let body: EsSearchRoot<I> = reqwest::get(&route)
                                .map_err(|e| EcError::ReqwestGetError(format!("{}",e)))?
                                .json()
                                .map_err(|e| EcError::ReqwestJsonError(format!("{}",e)))?;

        // extract a vector of the supplied type from the json returned by
        // the query and return the results.
        let res = body.hits.hits.into_iter().map(|x| x.source).collect::<Vec<I>>();
        Ok(res)
    }

    /// Delete one or more indices, provided as a string
    ///
    /// # Parameters
    ///
    /// * `indices` - Reference to a `Vector` Of Index instances
    ///
    /// # Returns
    ///
    /// * `request::Response` instance if successful
    /// * `EcError` instance if unsuccessful
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

    // Build a uri given the resource
    fn get_route(&self, resource: &str) -> String {
        format!("http://{}:{}/{}", self.host, self.port, resource)
    }

}

