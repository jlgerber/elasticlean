extern crate elasticlean;
extern crate chrono;

use elasticlean::index::Index;
use chrono::NaiveDate;
use chrono::Utc;
use chrono::Datelike;

use elasticlean::elastic::*;

fn main() -> Result<(),String> {
    let idxstr = "foo-2018.03.02";
    let idx = Index::from_str(idxstr)?;
    println!("{}",idx);
    println!("name {}", idx.name);
    println!("date: {}", idx.date);
    let date = NaiveDate::from_ymd(2018, 3, 3);
    let offset = date.signed_duration_since(*idx.date());
    println!("days since {} = {}", idxstr, offset.num_days());

    let idxstr = "foo-2018.10.10";
    let idx = Index::from_str(idxstr)?;
    let now = Utc::now();
    println!("");
    println!("current date {}", now);
    let now_naive = NaiveDate::from_ymd(now.year(), now.month(), now.day());
    let offset = now_naive.signed_duration_since(*idx.date());
    println!("days since {} = {}", idxstr, offset.num_days());

    let ec = Elasticleaner::new("cs-elastic-client-01.d2.com",9200);
    let results: Vec<Index> = ec.get_indices().unwrap().into_iter().map(|v| Index::from_str(v.index.as_str()).unwrap()).collect();
    println!("{:?}", results);
    Ok(())
}
