use crate::{ConsumableToken, Parsable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorType<T>
where
    T: ConsumableToken,
{
    UnexpectedToken {
        expected: T,
        found: T,
    },
    NoMoreTokens,
    ParsedButUnmatching {
        err_msg: String,
    }, // TODO: specify extra fields here which might be useful
    ConjunctBranchParsingFailure {
        type_name: &'static str,
        err_source: Box<ParseError<T>>,
    },
    DisjunctBranchParsingFailure {
        type_name: &'static str,
        err_source: Vec<ParseError<T>>,
    },
}

// TODO: Refactor type_name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError<T>
where
    T: ConsumableToken,
{
    failed_at: usize,
    pub failure_type: ParseErrorType<T>,
}

impl<T> ParseError<T>
where
    T: ConsumableToken,
{
    pub fn new(
        failed_at: usize,
        failure_type: ParseErrorType<T>,
    ) -> Self {
        Self {
            failed_at,
            failure_type,
        }
    }

    pub fn from_conjunct_error(type_name: &'static str, other: ParseError<T>) -> Self {
        ParseError {
            failed_at: other.failed_at,
            failure_type: ParseErrorType::ConjunctBranchParsingFailure {
                type_name,
                err_source: Box::new(other),
            },
        }
    }

    pub fn parsed_but_unmatching<P>(failed_at: usize, result: &P) -> Self
    where
        P: Parsable<T>,
    {
        // TODO: Add pattern information here
        let err_msg = format!("Parsed {:?}, but it did not match pattern", result);
        ParseError {
            failed_at,
            failure_type: ParseErrorType::ParsedButUnmatching { err_msg },
        }
    }

    pub fn no_more_tokens(failed_at: usize) -> Self {
        ParseError {
            failed_at,
            failure_type: crate::ParseErrorType::NoMoreTokens,
        }
    }

    pub fn from_disjunct_errors(
        failed_at: usize,
        err_source: Vec<ParseError<T>>,
        type_name: &'static str,
    ) -> ParseError<T>
    {
        ParseError {
            failed_at,
            failure_type: ParseErrorType::DisjunctBranchParsingFailure {
                type_name,
                err_source,
            },
        }
    }

    pub fn to_string(&self, identation_level: usize) -> String {
        let tabs = "\t".repeat(identation_level);
        match &self.failure_type {
            ParseErrorType::UnexpectedToken { expected, found } => {
                let more_tabs = "\t".repeat(identation_level + 1);
                format!("{tabs}Unexpected Token Erorr\n{more_tabs}Expected {:?}\n{more_tabs}Found {:?}\n", expected, found)
            }
            ParseErrorType::NoMoreTokens => {
                format!("{tabs}Ran out of tokens\n")
            }
            ParseErrorType::ParsedButUnmatching { err_msg } => {
                format!("{tabs}{err_msg}")
            }
            ParseErrorType::ConjunctBranchParsingFailure {
                type_name,
                err_source,
            } => {
                let err_source_str = err_source.to_string(identation_level + 1);
                format!("{tabs}Failed to parse {type_name}:\n{err_source_str}")
            }
            ParseErrorType::DisjunctBranchParsingFailure {
                type_name,
                err_source,
            } => {
                let errors = err_source
                    .iter()
                    .map(|e| e.to_string(identation_level + 1))
                    .reduce(|accum, curr| accum + "\n" + &curr)
                    .expect("Enums without variants cannot implement Parsable");
                format!("{tabs}Failed to parse {type_name}:\n{errors}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{t, ParseError};
    use crate::token::Token;

    #[test]
    fn to_string() {
        let result = ParseError::from_conjunct_error(
            "ConjunctType",
            ParseError::from_disjunct_errors(
                1,
                vec![
                    ParseError::from_conjunct_error(
                        "SubType1",
                        ParseError::parsed_but_unmatching(1, &t!(return)),
                    ),
                    ParseError::parsed_but_unmatching(1, &t!(litint 3)),
                ],
                "DisjunctType",
            ),
        ).to_string(0);
        let expected = format!("Failed to parse ConjunctType:
\tFailed to parse DisjunctType:
\t\tFailed to parse SubType1:
\t\t\tParsed {:?}, but it did not match pattern
\t\tParsed {:?}, but it did not match pattern", t!(return), t!(litint 3));
        println!("{result}");
        assert_eq!(expected, result)
    }
}
