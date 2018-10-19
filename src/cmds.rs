use std::collections::HashSet;

use errors::EcError;
use index::Index;
use elastic::Elasticleaner;
use constants;

// Given optional name, start, and end, return a Result wrapped
// vector of Index structs if successful, or an EcError in the failure case.
// The start and end are expressed in age in days from today, and can
// express the interval [s,e).
pub fn get_indices(name: Option<String>, start: Option<i32>, end: Option<i32>)
-> Result<Vec<Index>, EcError> {
    let ec = Elasticleaner::new("cs-elastic-client-01.d2.com",9200);
    let  r = ec.get_indices()?.into_iter();
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

// process the query subcommand
pub fn process_query(name: Option<String>, start: Option<i32>, end: Option<i32>, names_only:bool)
-> Result<(), EcError> {


    let mut results: Vec<Index> = get_indices(name, start, end)?;
    let sz = results.len();

    if names_only {
        let mut seen: HashSet<&str> = HashSet::new();
        for r in &results {
            if !seen.contains(r.name()) {
                let n = r.name().clone();
                seen.insert(n);
                println!("{}", n);
            }
        }
    } else {
        results.sort_unstable();
        let mut last = String::new();
        for r in results {
            // add space between different base index names
            if last != r.name {
                last = r.name.clone();
                println!("");
            }

            println!("{}-{}", r.name, r.date);
        }
    }

    println!("{} indices", sz);

    Ok(())
}

// process the delete subcommand
pub fn process_delete(name: String, start: Option<i32>, end: i32, dry_run: bool)
 -> Result<(), EcError> {

    // make sure that we keep the minimum number of indices no matter what the user
    // requests
    let end_new = if end > constants::MIN_DAYS { end } else {
        debug!("process_delete requested end value {} falls within MIN_DAYS.
         Using MIN_DAYS {} ",
        end, constants::MIN_DAYS);
        constants::MIN_DAYS
    };

    let results: Vec<Index> = get_indices(Some(name.clone()), start, Some(end_new))?;

    let sz = results.len();
    let idxs = results.into_iter()
        .map(|i| format!("{}",i))
        .collect::<Vec<String>>()
        .join(",");

    // apply
    println!("joined indices for delete: {} ",idxs);
    println!("{} indices will be deleted", sz);
    if dry_run {
        println!("dry-run");
        return Ok(());
    }

    let ec = Elasticleaner::new("cs-elastic-client-01.d2.com",9200);

    let results = ec.delete_indices(idxs);
    println!("delete results: {:#?}", results);
    Ok(())
}