//! cmds.rs
//!
//! Convenience struct which defines methods for cli
use crate::cmdprocessor::CmdProcessor;
use crate::errors::EcError;
use crate::indices::Deprecate;
use crate::traits::ElasticIndex;

/// Provides cli command methods
pub struct Cmds {
    processor: CmdProcessor
}

impl Cmds {
    /// Construct a new Cmds struct, wrapping the CmdProcessor, which does the heavy
    /// lifting on the Cmds behalf.
    pub fn new(cproc: CmdProcessor) -> Cmds {
        Cmds {
            processor: cproc
        }
    }

    /// Query for indices matching age criteria and print out results.
    ///
    /// # Arguments
    ///
    /// * `name` - an optional string which is the name of the index we want to query
    /// * `start` - an optional start offset, in days, from today
    /// * `end` - an optional end offset, in days, from today
    /// * `names_only` - whether to query the names of the indices or a specific name
    ///
    /// # Returns
    ///
    /// * `()` - On success
    /// * `EcError` - On failure
    // TODO: the call is a bit odd. We could change this to multiple methods (query_index, index_names)
    // or we could take an enum QueryParams { Range{name,start,end}, Names }
    pub fn query(&self, name: Option<String>, start: Option<i32>, end: Option<i32>, names_only:bool)
    -> Result<(), EcError> {

        let  results = if names_only {
            self.processor.query_names(name, start, end)?
        } else {
            self.processor.query(name, start, end)?
        };

        for r in &results {
                println!("{}", r);
        }

        println!("Number of Indices: {}",results.len());

        Ok(())
    }

    /// The process subcommand retrieves an index, optionally with a start and
    /// end offset, and matches its name against known indices. If the index is
    /// known to the system, it retrieves values, possibly within the optional
    /// offset times, and prints out a result, returning `()`. Otherwise, it
    /// returns a ParseError.
    ///
    /// # Arguments
    ///
    /// * `name` - the name of the index
    /// * `start` - optional offset start time, in days
    /// * `end` - optional offset end time for the query, in days
    ///
    /// Note start must be greater than end if provided
    ///
    /// # Returns
    ///
    /// * `()` - On Success
    /// * `EcError` - On Failure
    pub fn process(&self, name: String, start: Option<i32>, end: Option<i32>)
    -> Result<(), EcError> {

        let results = match name.as_str() {
            Deprecate::NAME => { self.processor.get::<Deprecate>(start, end) },
            _ => {
                Err(EcError::ParseError(format!("Unrecognized index: {}", name)))
            }
        }?;

        for r in &results {
            println!("{}", r);
        }

        Ok(())
    }
    /// The delete command removes the provided index values, optionally, beginning
    /// at a start offsent, and ending `end` days before now. There is a dry run mode
    /// provided as well.AsMut
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the index
    /// * `start` - Optional start offset, from today, of the deletion
    /// * `end` - The end offset of the deletion
    ///
    /// # Returns
    ///
    /// * `()` - on Success
    /// * `EcError` - on Failure
    pub fn delete(&self, name: String, start: Option<i32>, end: i32, dry_run: bool)
    -> Result<(), EcError> {

        self.processor.delete(name, start, end, dry_run)

    }
}