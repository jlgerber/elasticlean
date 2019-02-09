//! Convenience struct which defines methods
use crate::cmdprocessor::CmdProcessor;
use crate::errors::EcError;
use crate::indices::Deprecate;
use crate::traits::ElasticIndex;

pub struct Cmds {
    processor: CmdProcessor
}

impl Cmds {
    pub fn new(cproc: CmdProcessor) -> Cmds {
        Cmds {
            processor: cproc
        }
    }

// process the query subcommand
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

// process the process subcommand
pub fn process(&self, name: String,start: Option<i32>, end: Option<i32>)
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
// process the delete subcommand
pub fn delete(&self, name: String, start: Option<i32>, end: i32, dry_run: bool)
 -> Result<(), EcError> {

    self.processor.delete(name, start, end, dry_run)

}

}