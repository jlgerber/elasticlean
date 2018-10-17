use std::fmt::Display;
use std::default::Default;
use chrono::naive::NaiveDate;
use chrono::Utc;
use chrono::Datelike;
use std::num::ParseIntError;
use std::fmt;

use indexparser::IndexParser;

/// The return type
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Index {
    pub name: String,
    pub date: NaiveDate
}

// return a string formatted thusly: ```base-YYYY.MM.DD```
impl Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.name, self.date.to_string().replace("-","."))
    }
}

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

    /// Given a str return a result
    pub fn from_str(name: &str) -> Result<Index, String> {
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
