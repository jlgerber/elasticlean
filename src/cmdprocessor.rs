use constants;
use elasticrud::Elasticrud;
use errors::EcError;
use index::Index;
use std::collections::HashSet;
use traits::ElasticIndex;

/// Process commands
pub struct CmdProcessor {
    pub ec: Elasticrud,
}

impl CmdProcessor {
    /// new up a CmdProcessor
    pub fn new(host: &str, port: u16) -> CmdProcessor
    {
       CmdProcessor {
        ec: Elasticrud::new(host, port)
       }
    }

    // Given optional name, start, and end, return a Result wrapped
    // vector of Index structs if successful, or an EcError in the failure case.
    // The start and end are expressed in age in days from today, and can
    // express the interval [s,e).
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


    /// process the query subcommand
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


    /// process the query subcommand
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

    // get vector of Indexes implementing the ElasticIndex trait.
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

    // process the delete subcommand
    pub fn delete(&self, name: String, start: Option<i32>, end: i32, dry_run: bool)
    -> Result<(), EcError> {

        // make sure that we keep the minimum number of indices no matter what the user
        // requests
        let end_new = if end > constants::MIN_DAYS { end } else {
            debug!("process_delete requested end value {} falls within MIN_DAYS.
            Using MIN_DAYS {} ",
            end, constants::MIN_DAYS);
            constants::MIN_DAYS
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
