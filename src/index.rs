use std::fmt::Display;
use std::default::Default;
use chrono::naive::NaiveDate;
use std::num::ParseIntError;
use std::fmt;

/// The return type
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Index {
    pub name: String,
    pub date: NaiveDate
}

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
    pub fn new<I>(name: I, year: i32, month: u32, day: u32) -> Index
    where I: Into<String>
    {
        Index {
            name:  name.into(),
            date: NaiveDate::from_ymd(year, month, day)
        }
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
}
