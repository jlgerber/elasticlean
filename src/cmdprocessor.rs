//! cmdprocessor.rs
//!
//! provides a CmdProcessor struct which handles interfacing with
//! elasticsearch, exposing commands to perform various tasks related to cleanup.
use crate::constants;
use crate::elasticrud::Elasticrud;
use crate::errors::EcError;
use crate::index::Index;
use std::collections::HashSet;
use crate::traits::ElasticIndex;
use crate::config::Config;

/// Struct responsible for executing commands
pub struct CmdProcessor<'a, 'b: 'a> {
    pub config: &'b Config,
    pub ec: Elasticrud<'a>,
}

impl<'a, 'b> CmdProcessor<'a, 'b> {
    /// new up a CmdProcessor
    ///
    /// # Arguments
    ///
    /// * `host` - A non owned host name
    /// * `port` - The port number
    ///
    /// # Returns
    ///
    /// * `CmdProcessor` instance
    pub fn new(config: &'b Config) -> CmdProcessor<'a, 'b>
    {
        let port  = config.port;
       CmdProcessor {
           config: config,
        ec: Elasticrud::new(&config.host, port)
       }
    }

    /// Given optional name, start, and end, return a Result wrapped
    /// vector of Index structs if successful, or an EcError in the failure case.
    /// The start and end are expressed in age in days from today, and can
    /// express the interval [s,e).
    ///
    /// # Arguments
    ///
    /// * `name`  - An optional index name
    /// * `start` - An optional start offset, from today
    /// * `end`   - An optional end offset, from today
    ///
    /// # Returns
    ///
    /// * Vector of `Index` instances, on Success
    /// * `ExError` on Failure
    pub fn get_indices(&self, name: Option<String>, start: Option<i32>, end: Option<i32>)
    -> Result<Vec<Index>, EcError> {

        let  r = self.ec.get_raw_indices()?.into_iter();
        // filter_map performs transformation, keeping any successful Result
        let  r = r.filter_map(|v| Index::from_str(v.index.as_str()).ok());
        // if name is supplied match against it
        let r = r.filter(|v| if let Some(ref n) = name { v.name == *n } else {true});
        // if start is supplied filter out any index that is loder than supplied start
        let r = r.filter(|v| if let Some(n) = start {v.days() <= n.into() } else {true});
        // if end is supplied, filter out any index which is newer than end
        let r = r.filter(|v| if let Some(n) = end {v.days() > n.into() } else {true});

        Ok(r.collect::<Vec<Index>>())
    }


    /// Get the names of all indeices matching criteria
    ///
    /// # Arguments
    ///
    /// * `name`  - Optional index name
    /// * `start` - Optional starting offset, from today
    /// * `end`   - Optional ending offset, from today
    ///
    /// # Returns
    ///
    /// * Vector of `String` upon success
    /// * EcError upon failure
    pub fn query_names(&self, name: Option<String>, start: Option<i32>, end: Option<i32>)
    -> Result<Vec<String>, EcError> {

        let  results: Vec<Index> = self.get_indices(name, start, end)?;

        let mut seen: HashSet<String> = HashSet::new();

        for r in &results {
            if !seen.contains(r.name()) {
                let n = r.name().to_string();
                seen.insert(n);
            }
        }

        let return_results = seen.drain().collect::<Vec<String>>();
        return Ok(return_results)
    }


    /// Retreve the dated index names matching the query parameters
    ///
    /// # Arguments
    ///
    /// `name`  - Optional base name string (sans date) of the index
    /// `start` - Optional starting offset, in days, from today (where start > end)
    /// `end`   - Optional ending offset, in days, from today (where end < start)
    ///
    /// # Returns
    ///
    /// * `Vector` of `String`s of the form <name>-<date>, if successful
    /// * `ExError` instance
    pub fn query(&self, name: Option<String>, start: Option<i32>, end: Option<i32>)
    -> Result<Vec<String>, EcError> {

        let mut results: Vec<Index> = self.get_indices(name, start, end)?;
        results.sort_unstable();
        let return_results =
            results.iter()
            .map(|r| format!("{}-{}", r.name, r.date))
            .collect::<Vec<String>>();

        return Ok(return_results);
    }

    /// Get vector of Indices each implementing the ElasticIndex trait and
    /// matching the optional criteria.
    ///
    /// # Parameters
    ///
    /// * `start` - Optional number of days prior to today to retrieve indices for
    /// * `end`   - Optional number of days prior to today to end retrieval of indices for
    ///
    /// # Returns
    ///
    /// * `Vector` of `ElasitcIndex`s if successful
    /// * `EcError` instance if failed
    pub fn get<I>(&self, start: Option<i32>, end: Option<i32>)
    -> Result<Vec<I>, EcError>
    where
        I: ElasticIndex
    {
        // retrive a vector of indices
        let mut indices: Vec<Index> =
            self.get_indices(Some(I::NAME.to_string()), start, end)?;
        //sort them
        indices.sort_unstable();
        // retrieve the results
        let results = self.ec.get::<I>(&indices)?;
        Ok(results)
    }

    /// Delete indices which match supplied criteria
    ///
    /// # Parameters
    ///
    /// * `name`    - Base name of index (sans date) we are interested in
    /// * `start`   - Optional starting offset in days to begin search
    /// * `end`     - Optional ending offset in days to end search
    /// * `dry_run` - Boolean indicating whether to actually perform
    ///               the delete operation or only report on what would
    ///               get deleted if run outside of dry_run mode
    ///
    /// # Returns
    ///
    /// * `()` if successful
    /// * `EcError`instance if unsuccessful
    pub fn delete(&self, name: String, start: Option<i32>, end: i32, dry_run: bool)
    -> Result<(), EcError> {

        // make sure that we keep the minimum number of indices no matter what the user
        // requests
        let end_new = if end > self.config.min_days as i32 { end } else {
            debug!("process_delete requested end value {} falls within MIN_DAYS.
            Using MIN_DAYS {} ", end, self.config.min_days);

            self.config.min_days as i32
        };

        let results: Vec<Index> =
            self.get_indices(Some(name.clone()), start, Some(end_new))?;

        if dry_run {
            let sz = results.len();
            let idxs = results.into_iter()
                .map(|i| format!("{}",i))
                .collect::<Vec<String>>()
                .join(",");

            println!("joined indices for delete: {} ",idxs);
            println!("{} indices will be deleted", sz);
            println!("dry-run");
        } else {
            let results = self.ec.delete_indices(&results);
            info!("delete results: {:#?}", results);
        }

        Ok(())
    }
}
