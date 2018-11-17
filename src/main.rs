#[macro_use]
extern crate log;
//#[macro_use]
extern crate structopt;

extern crate chrono;
extern crate elasticlean;
extern crate env_logger;
extern crate failure;

use elasticlean::{
    errors::EcError,
    cmds::*,
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "elasticlean")]
/// The elasticlean cli provides utilities to inspect and clean up Elasticsearch
/// indices.
///
/// Logging
///
/// The elasticlean cli logger may be controlled via the RUST_LOG
/// environment variable. Levels may be set globally
/// ( eg RUST_LOG=debug ), or in a more targetted fashion
/// ( eg RUST_LOG=elasticlean=debug ).
/// The latter is probably preferred under normal conditions if one wants to
/// avoid logging of dependent libraries. Note that the idiomatic way of
/// effecting log levels is to prefix the elasticlean command with RUST_LOG,
/// eg RUST_LOG=elasticlean=debug elasticlean query -s 5 -o
enum Opt {
    #[structopt(name = "query")]
    /// query indices
    Query {
        #[structopt(short = "n", long = "basename")]
        /// Specify the base name of the index. (sans date)
        name: Option<String>,

        #[structopt(short = "s", long = "start")]
        /// Specify the number of days back you want to start
        start: Option<i32>,

        #[structopt(short = "e", long = "end")]
        /// Specify the number of days back you want to stop
        end: Option<i32>,

        #[structopt(short = "o", long = "names-only")]
        /// Prints a list of unique base names of indices
        names_only: bool,
    },
    #[structopt(name = "process")]
    /// apply a process to indices
    Process {
        #[structopt(short = "n", long = "basename")]
        /// Specify the base name of the index. (sans date)
        name: String,

        #[structopt(short = "s", long = "start")]
        /// Specify the number of days back you want to start
        start: Option<i32>,

        #[structopt(short = "e", long = "end")]
        /// Specify the number of days back you want to stop
        end: Option<i32>,
    },
    #[structopt(name = "delete")]
    /// query indices
    Delete {
        #[structopt(short = "n", long = "basename")]
        /// Specify the base name of the index. (sans date)
        name: String,

        #[structopt(short = "s", long = "start")]
        /// Specify the number of days back you want to start
        start: Option<i32>,

        #[structopt(short = "e", long = "end")]
        /// Specify the number of days back you want to stop
        end: i32,

        #[structopt(short = "d", long = "dry-run")]
        dry_run: bool,
    },
}


fn main() -> Result<(), EcError> {
    env_logger::init();
    debug!("logger initialized");
    let matches = Opt::from_args();

    match matches {
         Opt::Query{ name, start, end, names_only } => process_query(name, start, end, names_only),
         Opt::Process{ name, start, end }           => process_process(name, start, end),
         Opt::Delete{ name, start, end, dry_run }   => process_delete(name, start, end, dry_run),
    }?;

    Ok(())
}
