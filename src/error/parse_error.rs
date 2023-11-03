use crossterm::style::Stylize;

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
    },
    ConjunctBranchParsingFailure {
        successes: Vec<String>,
        err_source: Box<ParseError<T>>,
    },
    DisjunctBranchParsingFailure {
        err_source: Vec<ParseError<T>>,
    },
}

// TODO: Refactor type_name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError<T>
where
    T: ConsumableToken,
{
    type_name: &'static str,
    failed_at: usize,
    pub failure_type: ParseErrorType<T>,
}

impl<T> ParseError<T>
where
    T: ConsumableToken,
{
    pub fn new<P>(failed_at: usize, failure_type: ParseErrorType<T>) -> Self
    where
        P: Parsable<T>,
    {
        Self {
            type_name: P::identifier(),
            failed_at,
            failure_type,
        }
    }

    pub fn parsed_but_unmatching<P>(
        failed_at: usize,
        result: &P,
        pattern: Option<&'static str>,
    ) -> Self
    where
        P: Parsable<T>,
    {
        let pattern = pattern.unwrap_or("(pattern not provided by user)");
        let type_name = <P as Parsable<T>>::identifier();
        let err_msg = format!(
            "Parsed {:?}: {type_name}, but it did not match pattern '{pattern}'",
            result
        );
        ParseError::new::<P>(failed_at, ParseErrorType::ParsedButUnmatching { err_msg })
    }

    pub fn no_more_tokens<P>(failed_at: usize) -> Self
    where
        P: Parsable<T>,
    {
        ParseError::new::<P>(failed_at, ParseErrorType::NoMoreTokens)
    }

    pub fn from_conjunct_error<P>(other: ParseError<T>, sucessses: &[String]) -> Self
    where
        P: Parsable<T>,
    {
        ParseError::new::<P>(
            other.failed_at,
            ParseErrorType::ConjunctBranchParsingFailure {
                successes: vec![],
                err_source: Box::new(other),
            },
        )
    }

    pub fn from_disjunct_errors<P>(failed_at: usize, err_source: Vec<ParseError<T>>) -> Self
    where
        P: Parsable<T>,
    {
        ParseError::new::<P>(
            failed_at,
            ParseErrorType::DisjunctBranchParsingFailure { err_source },
        )
    }

    pub fn stringify(&self, indentation_level: usize) -> String {
        let tabs = "\t".repeat(indentation_level);
        match &self.failure_type {
            ParseErrorType::UnexpectedToken { expected, found } => {
                let more_tabs = "\t".repeat(indentation_level + 1);
                format!("{tabs}Unexpected Token Error\n{more_tabs}Expected {:?}\n{more_tabs}Found {:?}\n", expected, found)
            }
            ParseErrorType::NoMoreTokens => {
                format!("{tabs}Ran out of tokens\n")
            }
            ParseErrorType::ParsedButUnmatching { err_msg } => {
                format!("{tabs}{err_msg}")
            }
            ParseErrorType::ConjunctBranchParsingFailure { err_source, .. } => {
                let err_source_str = err_source.stringify(indentation_level + 1);
                format!(
                    "{tabs}Failed to parse {}:\n{err_source_str}",
                    self.type_name
                )
            }
            ParseErrorType::DisjunctBranchParsingFailure { err_source } => {
                let errors = err_source
                    .iter()
                    .map(|e| e.stringify(indentation_level + 1))
                    .reduce(|accum, curr| accum + "\n" + &curr)
                    .expect("Enums without variants cannot implement Parsable");
                format!("{tabs}Failed to parse {}:\n{errors}", self.type_name)
            }
        }
    }
}

impl<T: ConsumableToken> std::fmt::Display for ParseError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "red".red())
        // write!(f,"{}",self.stringify(0))
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::{t, Parsable, ParseError};

    #[test]
    fn a() {
        let error = ParseError::from_conjunct_error::<Token>(
            ParseError::from_disjunct_errors::<Token>(
                1,
                vec![
                    ParseError::parsed_but_unmatching::<Token>(1, &t!(return), Some("hahah ")),
                    ParseError::parsed_but_unmatching::<Token>(1, &t!(litint 3), Some("heeh")),
                ],
            ),
            vec![].as_slice(),
        );
        println!("{error}")
    }

    #[test]
    fn to_string() {
        let pattern1 = "Token::KwReturn";
        let pattern2 = "Token::LiteralInt(_)";
        let result = ParseError::from_conjunct_error::<Token>(
            ParseError::from_disjunct_errors::<Token>(
                1,
                vec![
                    ParseError::parsed_but_unmatching::<Token>(1, &t!(return), Some(pattern1)),
                    ParseError::parsed_but_unmatching::<Token>(1, &t!(litint 3), Some(pattern2)),
                ],
            ),
            vec![].as_slice(),
        )
        .stringify(0);
        let expected_identifier = <Token as Parsable<Token>>::identifier();
        let expected = format!(
            "Failed to parse {expected_identifier}:
\tFailed to parse {expected_identifier}:
\t\tParsed {:?}: {expected_identifier}, but it did not match pattern '{pattern1}'
\t\tParsed {:?}: {expected_identifier}, but it did not match pattern '{pattern2}'",
            t!(return),
            t!(litint 3),
        );
        assert_eq!(expected, result)
    }
}
