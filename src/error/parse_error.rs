
use crate::base_traits::{Parsable, ConsumableToken};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorType
{
    NoMoreTokens,
    ParsedButUnmatching {
        err_msg: String,
    },
    ConjunctBranchParsingFailure {
        successes: Vec<String>,
        err_source: Box<ParseError>,
    },
    DisjunctBranchParsingFailure {
        err_source: Vec<ParseError>,
    },
}

// TODO: Refactor type_name
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError
{
    type_name: &'static str,
    failed_at: usize,
    pub failure_type: ParseErrorType,
}


pub fn identifier<P>() -> &'static str {
    std::any::type_name::<P>()
}

impl ParseError

{
    pub fn new(type_name: &'static str , failed_at: usize, failure_type: ParseErrorType) -> Self
    {
        Self {
            type_name,
            failed_at,
            failure_type,
        }
    }

    pub fn parsed_but_unmatching< T, P>(
        failed_at: usize,
        result: &P,
        pattern: & str,
    ) -> Self
    where
        T: ConsumableToken,
        P: Parsable<T>,

    {
        let type_name = identifier::<P>();
        let err_msg = format!(
            "Parsed {:?}: {type_name}, but it did not match pattern '{pattern}'",
            result
        );
        ParseError::new(type_name, failed_at, ParseErrorType::ParsedButUnmatching { err_msg })
    }

    pub fn no_more_tokens<T:ConsumableToken>(failed_at: usize) -> Self
    {
        ParseError::new(identifier::<T>(), failed_at, ParseErrorType::NoMoreTokens)
    }

    pub fn from_conjunct_error<P>(other: ParseError, successes: Vec<String>) -> Self
    {
        ParseError::new(
            identifier::<P>(),
            other.failed_at,
            ParseErrorType::ConjunctBranchParsingFailure {
                successes,
                err_source: Box::new(other),
            },
        )
    }

    pub fn from_disjunct_errors<P>(failed_at: usize, err_source: Vec<ParseError>) -> Self
    {
        ParseError::new(
            identifier::<P>(),
            failed_at,
            ParseErrorType::DisjunctBranchParsingFailure { err_source },
        )
    }

    pub fn stringify(&self, indentation_level: usize) -> String {
        let tabs = "\t".repeat(indentation_level);
        match &self.failure_type {
            ParseErrorType::NoMoreTokens => {
                format!("{tabs}Ran out of tokens\n")
            }
            ParseErrorType::ParsedButUnmatching { err_msg } => {
                format!("{tabs}{err_msg}")
            }
            ParseErrorType::ConjunctBranchParsingFailure {
                err_source,
                successes,
            } => {
                let err_source_str = err_source.stringify(indentation_level + 1);
                let inner_tabs = "\t".repeat(indentation_level + 1);
                let successes = successes
                    .iter()
                    .map(|s| format!("{inner_tabs} Success: {s}"))
                    .reduce(|accum, cur| accum + "\n" + &cur)
                    .unwrap_or("".to_string());

                format!(
                    "{tabs}Failed: {}:\n{successes}\n{err_source_str}",
                    self.type_name
                )
            }
            ParseErrorType::DisjunctBranchParsingFailure { err_source } => {
                let errors = err_source
                    .iter()
                    .map(|e| e.stringify(indentation_level + 1))
                    .reduce(|accum, curr| accum + "\n" + &curr)
                    .expect("Enums without variants cannot implement Parsable");
                format!("{tabs}Failed: {}:\n{errors}", self.type_name)
            }
        }
    }
}

impl  std::fmt::Display for ParseError 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.stringify(0))
        // write!(f,"{}",self.stringify(0))
    }
}

// #[cfg(test)]
// mod tests {

//     use crate::token::Token;
//     use crate::{t, Parsable, ParseError};

//     #[test]
//     fn a() {
//         let error = ParseError::from_conjunct_error::<Token>(
//             ParseError::from_disjunct_errors::<Token>(
//                 1,
//                 vec![
//                     ParseError::parsed_but_unmatching::<Token>(1, &t!(return), Some("hahah ")),
//                     ParseError::parsed_but_unmatching::<Token>(1, &t!(litint 3), Some("heeh")),
//                 ],
//             ),
//             vec![
//                 "Tokan::KReturn".to_owned(),
//                 "Token::LiteralInt(4)".to_owned(),
//             ]
//             .as_slice(),
//         );
//         println!("{error}")
//     }

//     #[derive(Debug)]
//     struct TestStruct {
//         // #[pattern(Token::LiteralInt(_))]
//         token_a: Token,
//         other_struct: OtherStruct,
//     }

//     #[derive(Debug)]
//     struct OtherStruct {
//         // #[pattern(Token::KReturn)]
//         token_b: Token,
//     }

//     impl Parsable<Token> for TestStruct {
//         fn parse(iter: &mut crate::TokenIter<Token>) -> Result<Self, ParseError<Token>> {
//             let other_struct = iter.parse()?;
//             let token_a = iter.parse_if_match(|t| matches!(t, Token::LiteralInt(_)), None)?;
//             Ok(TestStruct {
//                 other_struct,
//                 token_a,
//             })
//         }
//     }

//     impl Parsable<Token> for OtherStruct {
//         fn parse(iter: &mut crate::TokenIter<Token>) -> Result<Self, ParseError<Token>> {
//             let token_b = iter.parse_if_match(|t| matches!(t, Token::KReturn), None)?;
//             Ok(OtherStruct { token_b })
//         }
//     }

// //     #[test]
// //     fn conjunct_to_string() {
// //         let pattern1 = "Token::KReturn";
// //         let pattern2 = "Token::LiteralInt(_)";
// //         let pattern3 = "Token::LiteralString(\"Hello\")";
// //         let pattern4 = "Token::KwInt";
// //         let result = ParseError::from_conjunct_error::<TestStruct>(
// //             ParseError::from_conjunct_error::<OtherStruct>(
// //                 ParseError::parsed_but_unmatching(
// //                     1,
// //                     &Token::KReturn,
// //                     Some("Token::LiteralInt(_)"),
// //                 ),
// //                 vec![].as_slice(),
// //             ),
// //             vec!["Token::LiteralInt(_)"].as_slice(),
// //         )
// //         .stringify(0);
// //         let expected_identifier = <Token as Parsable<Token>>::identifier();
// //         let expected = format!(
// //             "Failed: {expected_identifier}:
// // \tFailed: {expected_identifier}:
// // \t\tParsed {:?}: {expected_identifier}, but it did not match pattern '{pattern1}'
// // \t\tParsed {:?}: {expected_identifier}, but it did not match pattern '{pattern2}'",
// //             t!(return),
// //             t!(litint 3),
// //         );
// //         assert_eq!(expected, result)
// //     }
// //     #[test]
// //     fn to_string() {
// //         let pattern1 = "Token::KReturn";
// //         let pattern2 = "Token::LiteralInt(_)";
// //         let pattern3 = "Token::LiteralString(\"Hello\")";
// //         let pattern4 = "Token::KwInt";
// //         let result = ParseError::from_conjunct_error::<Token>(
// //             ParseError::from_disjunct_errors::<Token>(
// //                 1,
// //                 vec![
// //                     ParseError::parsed_but_unmatching::<Token>(1, &t!(return), Some(pattern1)),
// //                     ParseError::parsed_but_unmatching::<Token>(1, &t!(litint 3), Some(pattern2)),
// //                 ],
// //             ),
// //             vec![pattern3.to_owned(), pattern4.to_owned()].as_slice(),
// //         )
// //         .stringify(0);
// //         let expected_identifier = <Token as Parsable<Token>>::identifier();
// //         let expected = format!(
// //             "Failed: {expected_identifier}:
// // \tFailed: {expected_identifier}:
// // \t\tParsed {:?}: {expected_identifier}, but it did not match pattern '{pattern1}'
// // \t\tParsed {:?}: {expected_identifier}, but it did not match pattern '{pattern2}'",
// //             t!(return),
// //             t!(litint 3),
// //         );
// //         assert_eq!(expected, result)
// //     }
// }
