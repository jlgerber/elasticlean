use pest::Parser;
use std::default::Default;
#[derive(Parser)]
#[grammar = "index.pest"]
struct _IndexParser;

pub struct IndexParser;

/// The return type
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Index {
    pub name: String,
    pub year: String, // change later
    pub month: String,
    pub day: String,
}

impl Index {
    // TODO: change year, month, day into u8 and return result
    pub fn new<I>(name: I, year: I, month: I, day: I) -> Index
    where I: Into<String>
    {
        Index {
            name:  name.into(),
            year:  year.into(),
            month: month.into(),
            day:   day.into(),
        }
    }
}

impl IndexParser {
    /// parse an elasticsearch index, of the form ```name-YYYY.MM.DD``` and return
    /// a Result- either an Ok Index nistance, or an Err String.
    pub fn parse(input: &str ) -> Result<Index, String> {
        let index =  _IndexParser::parse(Rule::index, input).map_err(|e| format!("{}",e))?;

        // Because ident_list is silent, the iterator will contain idents
        let mut idx = Index::default();

        for idx_piece in index {
            let span = idx_piece.clone().into_span();

            // A idx_piece can be converted to an iterator of the tokens which make it up:
            for inner_idx_piece in idx_piece.into_inner() {
                let inner_span = inner_idx_piece.clone().into_span();


                match inner_idx_piece.as_rule() {
                    Rule::base => {
                        idx.name = inner_span.as_str().to_string();
                    },
                    Rule::date => {
                        for date_piece in inner_idx_piece.into_inner() {
                            let inner_span = date_piece.clone().into_span();
                            match date_piece.as_rule() {
                                Rule::year  => {
                                    idx.year = inner_span.as_str().to_string();
                                },
                                Rule::month => {
                                    idx.month = inner_span.as_str().to_string();
                                },
                                Rule::day   => {
                                    idx.day = inner_span.as_str().to_string();
                                },
                                _ => unreachable!()
                            }
                        }
                    },
                    _ => unreachable!()
                };
            }
        }

        Ok(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_new() {
        let id = Index::new("foo", "2018", "02", "04");
        let expected = Index {
            name: "foo".to_string(),
            year: "2018".to_string(),
            month: "02".to_string(),
            day: "04".to_string(),
        };
        assert_eq!(id, expected);
    }

    #[test]
    fn index_parse() {
        let id = IndexParser::parse("foo-2018.02.22");
        let expected = Index {
            name: "foo".to_string(),
            year: "2018".to_string(),
            month: "02".to_string(),
            day: "22".to_string(),
        };
        assert_eq!(id, Ok(expected));
    }

    #[test]
    fn index_parse_out_of_range() {
        let id = IndexParser::parse("foo-2018.13.22");
        assert!(id.is_err());
    }

    #[test]
    fn index_parse_out_of_range2() {
        let id = IndexParser::parse("foo-2018.11.32");
        assert!(id.is_err());
    }

    #[test]
    fn index_parse_out_of_range_dates() {
        let id = IndexParser::parse("foo-2018.01.32");
        assert!(id.is_err());
    }
}
