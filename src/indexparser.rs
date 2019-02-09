//! # indexparser.rs
//!
//! Parser for the dated index name, generated in logstash. This uses
//! Pest transform a PEG grammar into a struct. The grammar file is located
//! with the source code, and named ```index.pest```
//!
use pest::Parser;
use index::Index;
use errors::EcError;

// The pest parser is not exposed directly.
#[derive(Parser)]
#[grammar = "index.pest"]
struct _IndexParser;

// IndexParser is a convenience struct which provides a parse method that is more suited
// to the api than the raw pest _IndexParser.

/// A dataless struct which provides an api for parsing an Index from an input &str
pub struct IndexParser;

impl IndexParser {
    /// parse an elasticsearch index, of the form ```name-YYYY.MM.DD``` and return
    /// a Result - either an Ok Index instance, or an Err String.
    pub fn parse(input: &str ) -> Result<Index, EcError> {
        let index =  _IndexParser::parse(Rule::index, input).map_err(|e| EcError::ParseError(format!("{}",e)))?;

        // parsing guarantees that these vars are going to get set. we just choose arbitrary
        // values for now.
        let mut name = "foo";
        let mut year = "2000";
        let mut month = "01";
        let mut day = "01";

        for idx_piece in index {

            // A idx_piece can be converted to an iterator of the tokens which make it up:
            for inner_idx_piece in idx_piece.into_inner() {
                let inner_span = inner_idx_piece.clone().into_span();

                match inner_idx_piece.as_rule() {
                    Rule::base => {
                        name = inner_span.as_str();
                    },
                    Rule::date => {
                        for date_piece in inner_idx_piece.into_inner() {
                            let inner_span = date_piece.clone().into_span();
                            match date_piece.as_rule() {
                                Rule::year  => {
                                    year = inner_span.as_str();
                                },
                                Rule::month => {
                                    month = inner_span.as_str();
                                },
                                Rule::day   => {
                                    day = inner_span.as_str();
                                },
                                _ => unreachable!()
                            }
                        }
                    },
                    _ => unreachable!()
                };
            }
        }

        let idx = Index::from_strs(name, year, month, day)
                    .map_err(|e| EcError::ParseError(format!("{}",e)))?;
        Ok(idx)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::naive::NaiveDate;


    #[test]
    fn index_parse() {
        let id = IndexParser::parse("foo-2018.02.22");
        let expected = Index {
            name: "foo".to_string(),
            date: NaiveDate::from_ymd(2018, 2, 22)
        };
        assert_eq!(id, Ok(expected));
    }

    #[test]
    fn index_parse_long() {
        let id = IndexParser::parse("foo-1.2.3-2018.02.22");
        let expected = Index {
            name: "foo-1.2.3".to_string(),
            date: NaiveDate::from_ymd(2018, 2, 22)
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
