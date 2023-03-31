use std::marker::PhantomData;

use crate::Parsable;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorType<T>
{
    ExpectationError(ExpectationError<T>),
    FailedToParseBranch(Box<ParseError<T>>),
}

pub enum ExpectationError<T> {
    UnexpectedToken {
        expected: T,
        found: T,
    },
    NoMoreTokens,
}

#[derive(Debug, Clone, Eq)]
pub struct ParseError<T>
{
    failed_at: usize,
    failure_type: ParseErrorType<T>,
    type_name: syn::Ident,
}





impl <P,T> ParseError<T> where P: Parsable<T> ,
T: Parsable<T>{
    pub fn from_failed_expectation(expectation_error: ExpectationError<T>) -> Self {
        ParseError { failed_at: (), failure_type: (), type_name: () }

    }

    pub fn from_branch_error(other: ParseError<T>) -> Self {
        let new_parse_error = other.clone();
        new_parse_error.failure_type = Box::new(other);
        new_parse_error.type_name = P::identifier();
        new_parse_error
    }

}