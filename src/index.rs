//! # index.rs
//!
//! representation of our elasticsearch index format,
//! consisting of a base name and date, generated
//! in logstash.

use chrono::{
    naive::NaiveDate,
    Utc,
    Datelike
};
use crate::errors::EcError;
use crate::indexparser::IndexParser;
use std::{
    default::Default,
    fmt,
    fmt::Display,
    num::ParseIntError,
};
use std::cmp::{PartialEq, Ordering}; // nested imports coming soon to rust

/// The Index struct is designed to handle dated indices
/// of the form ```NAME-YYYY.MM.DD```
/// The Index provides methods to create, sort, and present
/// the Index.
#[derive( Eq, Debug )]
pub struct Index {
    pub name: String,
    pub date: NaiveDate,
}

// Trait Impls for Comparisions
impl PartialEq for Index {
    fn eq(&self, other: &Index) -> bool {
        self.name == other.name && self.date == other.date
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Index) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Index {
    fn cmp(&self, other: &Index) -> Ordering {
        // if the names match then we compare on the date field
        if self.name == other.name {
            self.date.cmp(&other.date)
        } else {
            // otherwise, we simply compare on the name field
            self.name.cmp(&other.name)
        }
    }
}

// return a string formatted thusly: ```base-YYYY.MM.DD```
impl Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.date.to_string().replace("-","."))
    }
}
// TODO: pick a better default. Maybe the minimum date supported
impl Default for Index {
    fn default() -> Self {
        Index {
            name: String::new(),
            date: NaiveDate::from_ymd(2000, 1,1),
        }
    }
}

impl Index {
    // TODO: change year, month, day into u8 and return result
    /// Given a base name, year, month, and day, new up an Index
    pub fn new<I>(name: I, year: i32, month: u32, day: u32) -> Index
    where I: Into<String>
    {
        Index {
            name:  name.into(),
            date: NaiveDate::from_ymd(year, month, day)
        }
    }

    /// Given a str return a result wrapped Index or EcError.
    pub fn from_str(name: &str) -> Result<Index, EcError> {
        IndexParser::parse(name)
    }

    /// Given &str components, return a result that is either an index isntance or a ParseIntError
    pub fn from_strs<I>(name: I, year: &str, month: &str, day: &str) -> Result<Index, ParseIntError>
    where
        I: Into<String>
    {
        let year = year.parse::<i32>()?;
        let month = month.parse::<u32>()?;
        let day = day.parse::<u32>()?;
        Ok(Index {
            name: name.into(),
            date: NaiveDate::from_ymd(year, month, day)
        })
    }

    /// Get a reference to a str representing the base name of the Index
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get reference to a NaiveDate representing the date of the Index
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    /// Return the number of days old
    pub fn days(&self) -> i64 {
        let now = Utc::now();
        let now_naive = NaiveDate::from_ymd(now.year(), now.month(), now.day());
        let offset = now_naive.signed_duration_since(*self.date());
        offset.num_days()
    }

    /// Return the number of days since a Datelike input as an i64
    pub fn days_since<D>(&self, from_date: &D) -> i64
    where
        D: Datelike
    {
        let nd = NaiveDate::from_ymd(from_date.year(), from_date.month(), from_date.day());
        let offset = nd.signed_duration_since(*self.date());
        offset.num_days()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_new() {
        let id = Index::new("foo", 2018, 2, 4);
        let expected = Index {
            name: "foo".to_string(),
            date: NaiveDate::from_ymd(2018, 2, 4)
        };
        assert_eq!(id, expected);
    }

    #[test]
    #[should_panic]
    fn index_new_panic() {
        let id = Index::new("foo", 0, 0, 0);
        let expected = Index {
            name: "foo".to_string(),
            date: NaiveDate::from_ymd(0, 0, 0)
        };
        assert_eq!(id, expected);
    }

    #[test]
    fn index_from_str() {
        let id = Index::from_str("foo-2018.02.04");
        let expected = Index {
            name: "foo".to_string(),
            date: NaiveDate::from_ymd(2018, 2, 4)
        };
        assert_eq!(id, Ok(expected));
    }


    #[test]
    fn index_from_str3() {
        let id = Index::from_str("foo-1.2.3-2018.02.04");
        let expected = Index {
            name: "foo-1.2.3".to_string(),
            date: NaiveDate::from_ymd(2018, 2, 4)
        };
        assert_eq!(id, Ok(expected));
    }


    #[test]
    fn index_from_strs() {
        let id = Index::from_strs("foo", "2018", "02", "04");
        let expected = Index {
            name: "foo".to_string(),
            date: NaiveDate::from_ymd(2018, 2, 4)
        };
        assert_eq!(id, Ok(expected));
    }

    #[test]
    fn index_display() {
        let id = Index::from_strs("foo", "2018", "02", "04").unwrap();
        let idstr = format!("{}", id);
        let expected = "foo-2018.02.04".to_string();
        assert_eq!(idstr, expected);
    }

    #[test]
    fn index_name() {
        let id = Index::from_strs("foo", "2018", "02", "04").unwrap();
        let name = id.name();
        let expected = "foo";
        assert_eq!(name, expected);
    }

    #[test]
    fn index_date() {
        let id = Index::from_strs("foo", "2018", "02", "04").unwrap();
        let date = id.date();
        let expected = NaiveDate::from_ymd(2018,2,4);
        assert_eq!(date, &expected);
    }

    #[test]
    fn index_days_since() {
        let id = Index::from_strs("foo", "2018", "02", "03").unwrap();
        let fd = NaiveDate::from_ymd(2018,2,4);
        let days = id.days_since(&fd);
        assert_eq!(days, 1);
    }

    #[test]
    fn index_days_since2() {
        let id = Index::from_strs("foo", "2018", "02", "05").unwrap();
        let fd = NaiveDate::from_ymd(2018,2,4);
        let days = id.days_since(&fd);
        assert_eq!(days, -1);
    }

    #[test]
    fn index_lt() {
        let id = Index::from_str("foo-2018.02.05");
        let id2= Index::from_str("foo-2018.02.04");
        assert!(id > id2);
    }


    #[test]
    fn index_eq() {
        let id = Index::from_str("foo-2018.02.05");
        let id2= Index::from_str("foo-2018.02.05");
        assert_eq!(id, id2);
    }

    #[test]
    fn index_ne() {
        let id = Index::from_str("foo-2018.02.05");
        let id2= Index::from_str("foo-2017.02.05");
        assert_ne!(id, id2);
    }

    #[test]
    fn index_ne2() {
        let id = Index::from_str("foo-2018.02.05");
        let id2= Index::from_str("bar-2018.02.05");
        assert_ne!(id, id2);
    }
}
