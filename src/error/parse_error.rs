use crate::{ConsumableToken, Parsable};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorType<T>
where
    T: Parsable<T>,
{
    UnexpectedToken { expected: T, found: T },
    NoMoreTokens,
    UnmatchingToken { found: T, error_msg: Option<String> },
    ParsedButUnmatching, // TODO: specify extra fields here which might be useful
    ConjunctBranchParsingFailure(Box<ParseError<T>>),
    DisjunctBranchParsingFailure(Vec<ParseError<T>>),
}

// TODO: Refactor type_name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError<T>
where
    T: Parsable<T>,
{
    failed_at: usize,
    pub failure_type: ParseErrorType<T>,
    type_name: Option<String>,
}

impl<T> ParseError<T>
where
    T: Parsable<T>,
{
    pub fn new(
        failed_at: usize,
        failure_type: ParseErrorType<T>,
        type_name: Option<String>,
    ) -> Self {
        Self {
            failed_at,
            failure_type,
            type_name,
        }
    }
    pub fn from_conjunct_error(type_name: String, other: ParseError<T>) -> Self {
        ParseError {
            failed_at: other.failed_at,
            failure_type: ParseErrorType::ConjunctBranchParsingFailure(Box::new(other)),
            type_name: Some(type_name),
        }
    }

    pub fn parsed_but_unmatching(failed_at: usize) -> Self {
        ParseError {
            failed_at,
            failure_type: ParseErrorType::ParsedButUnmatching,
            type_name: None,
        }
    }

    pub fn unexpected_token(failed_at: usize, expected: T, found: T) -> Self {
        ParseError {
            failed_at,
            failure_type: crate::ParseErrorType::UnexpectedToken { expected, found },
            type_name: None,
        }
    }

    pub fn no_more_tokens(failed_at: usize) -> Self {
        ParseError {
            failed_at,
            failure_type: crate::ParseErrorType::NoMoreTokens,
            type_name: None,
        }
    }

    pub fn from_disjunct_errors(
        type_name: String,
        failed_at: usize,
        branches: Vec<ParseError<T>>,
    ) -> ParseError<T> {
        ParseError {
            failed_at,
            failure_type: ParseErrorType::DisjunctBranchParsingFailure(branches),
            type_name: Some(type_name),
        }
    }

    pub fn unmatching_token(failed_at: usize, error_msg: String, found: T) -> ParseError<T> {
        ParseError {
            failed_at,
            failure_type: ParseErrorType::UnmatchingToken {
                found,
                error_msg: Some(error_msg),
            },
            type_name: None,
        }
    }
}
