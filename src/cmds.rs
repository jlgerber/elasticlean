
use cmdprocessor::CmdProcessor;
use errors::EcError;
use indices::Deprecate;
use traits::ElasticIndex;

// process the query subcommand
pub fn process_query(name: Option<String>, start: Option<i32>, end: Option<i32>, names_only:bool)
-> Result<(), EcError> {

    let cproc = CmdProcessor::new("cs-elastic-client-01.d2.com", 9200);

    let  results = if names_only {
        cproc.query_names(name, start, end)?
    } else {
        cproc.query(name, start, end)?
    };

    for r in &results {
            println!("{}", r);
    }

    println!("Number of Indices: {}",results.len());

    Ok(())
}

// process the process subcommand
pub fn process_process(name: String,start: Option<i32>, end: Option<i32>)
-> Result<(), EcError> {

    let cproc = CmdProcessor::new("cs-elastic-client-01.d2.com", 9200);

    let results = match name.as_str() {
        Deprecate::NAME => { cproc.get::<Deprecate>(start, end) },
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
pub fn process_delete(name: String, start: Option<i32>, end: i32, dry_run: bool)
 -> Result<(), EcError> {

    let cproc = CmdProcessor::new("cs-elastic-client-01.d2.com", 9200);
    cproc.delete(name, start, end, dry_run)

}