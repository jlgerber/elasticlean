extern crate elasticlean;
extern crate chrono;
extern crate failure;

use elasticlean::{
    index::Index,
    errors::EcError
};

use elasticlean::elastic::*;

fn main() -> Result<(), EcError> {

    let ec = Elasticleaner::new("cs-elastic-client-01.d2.com",9200);
    let results: Vec<Index> = ec.get_indices()?
                                //.unwrap() // todo imp
                                .into_iter()
                                .filter_map(|v| Index::from_str(v.index.as_str()).ok())
                                .filter(|v| v.name == "exceptions")
                                .filter(|v| v.days() > 14)
                                .collect();
    println!("{:?}", results);

    /*
    // how would I go about doing this conditionally?
    // since the iterator is lazy, i can break this up
    let r = ec.get_indices()?.into_iter();
    let r = r.filter_map(|v| Index::from_str(v.index.as_str()).ok());
    let r = r.filter(|v| v.name == "exceptions");
    let r = r.filter(|v| v.days() > 14);
    let results: Vec<Index> = r.collect();
    println!("{:?}", results);
    */

    Ok(())
}
