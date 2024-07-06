use crate::{
    base_traits::{Matcher, Parsable, Parser},
    error::parse_error::{ParseError, ParseErrorType},
    identifier,
    iter::TokenIter,
    ConsumableToken,
};

impl<P1, P2, T> Parsable<T> for (P1, P2)
where
    P1: Parsable<T>,
    P2: Parsable<T>,
    T: ConsumableToken,
{
    fn parser() -> impl Parser<T, Self> {
        Tuple2Validator::default()
    }
}

struct Tuple2Validator<P1, P2>(Matcher<P1>, Matcher<P2>);

impl<P1, P2> Default for Tuple2Validator<P1, P2> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<T, P1, P2> Parser<T, (P1, P2)> for Tuple2Validator<P1, P2>
where
    P1: Parsable<T>,
    P2: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<(P1, P2), ParseError> {
        let p1: P1 = iter.parse::<P1>()?;
        let p2: P2 = iter.parse::<P2>()?;
        if (self.0)(&p1) && (self.1)(&p2) {
            Ok((p1, p2))
        } else {
            Err(ParseError::parsed_but_unmatching(
                iter.current,
                &(p1, p2),
                //TODO: add nicer error messages
                "Could not parse (P1, P2)",
            ))
        }
    }
}

impl<P1, P2, P3, T> Parsable<T> for (P1, P2, P3)
where
    P1: Parsable<T>,
    P2: Parsable<T>,
    P3: Parsable<T>,
    T: ConsumableToken,
{
    fn parser() -> impl Parser<T, Self> {
        Tuple3Validator::default()
    }
}

struct Tuple3Validator<P1, P2, P3>(Matcher<P1>, Matcher<P2>, Matcher<P3>);

impl<P1, P2, P3> Default for Tuple3Validator<P1, P2, P3> {
    fn default() -> Self {
        Self(Default::default(), Default::default(), Default::default())
    }
}

impl<T, P1, P2, P3> Parser<T, (P1, P2, P3)> for Tuple3Validator<P1, P2, P3>
where
    P1: Parsable<T>,
    P2: Parsable<T>,
    P3: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<(P1, P2, P3), ParseError> {
        let p1: P1 = iter.parse::<P1>()?;
        let p2: P2 = iter.parse::<P2>()?;
        let p3: P3 = iter.parse::<P3>()?;
        if (self.0)(&p1) && (self.1)(&p2) && (self.2)(&p3) {
            Ok((p1, p2, p3))
        } else {
            Err(ParseError::parsed_but_unmatching(
                iter.current,
                &(p1, p2, p3),
                "Could not parse (P1, P2, P3)",
            ))
        }
    }
}

impl<P1, P2, P3, P4, T> Parsable<T> for (P1, P2, P3, P4)
where
    P1: Parsable<T>,
    P2: Parsable<T>,
    P3: Parsable<T>,
    P4: Parsable<T>,
    T: ConsumableToken,
{
    fn parser() -> impl Parser<T, Self>
    where
        Self: Sized,
    {
        Tuple4Validator::default()
    }
}

struct Tuple4Validator<P1, P2, P3, P4>(Matcher<P1>, Matcher<P2>, Matcher<P3>, Matcher<P4>);

impl<P1, P2, P3, P4> Default for Tuple4Validator<P1, P2, P3, P4> {
    fn default() -> Self {
        Self(
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }
}

impl<T, P1, P2, P3, P4> Parser<T, (P1, P2, P3, P4)> for Tuple4Validator<P1, P2, P3, P4>
where
    P1: Parsable<T>,
    P2: Parsable<T>,
    P3: Parsable<T>,
    P4: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<(P1, P2, P3, P4), ParseError> {
        let p1: P1 = iter.parse::<P1>()?;
        let p2: P2 = iter.parse::<P2>()?;
        let p3: P3 = iter.parse::<P3>()?;
        let p4: P4 = iter.parse::<P4>()?;
        if (self.0)(&p1) && (self.1)(&p2) && (self.2)(&p3) && (self.3)(&p4) {
            Ok((p1, p2, p3, p4))
        } else {
            Err(ParseError::parsed_but_unmatching(
                iter.current,
                &(p1, p2, p3, p4),
                // TODO: add nice error message
                "Could not parse (P1, P2, P3, P4)",
            ))
        }
    }
}
// TODO: macro for tuple implementation

impl<T, P> Parsable<T> for Box<P>
where
    P: Parsable<T>,
    T: ConsumableToken,
{
    fn parser() -> impl Parser<T, Self>
    where
        Self: Sized,
    {
        BoxValidator::default()
    }
}

struct BoxValidator<P> {
    t: Matcher<P>,
}

impl<P> Default for BoxValidator<P> {
    fn default() -> Self {
        Self {
            t: Default::default(),
        }
    }
}

impl<T: ConsumableToken, P: Parsable<T>> Parser<T, Box<P>> for BoxValidator<P> {
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<Box<P>, ParseError> {
        let p = iter.parse::<P>()?;
        if (self.t)(&p) {
            Ok(Box::new(p))
        } else {
            Err(ParseError::parsed_but_unmatching(
                iter.current,
                &p,
                // TODO: add nice error message
                "Could not parse Option<P>",
            ))
        }
    }
}

impl<T, P> Parsable<T> for Vec<P>
where
    P: Parsable<T>,
    T: ConsumableToken,
{
    fn parser() -> impl Parser<T, Self> {
        VecValidator::default()
    }
}

struct VecValidator<P> {
    matcher: Matcher<P>,
}

impl<P> Default for VecValidator<P> {
    fn default() -> Self {
        Self {
            matcher: Default::default(),
        }
    }
}

impl<T: ConsumableToken, P: Parsable<T>> Parser<T, Vec<P>> for VecValidator<P> {
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<Vec<P>, ParseError> {
        let mut result = vec![];

        while let Ok(element) = iter.parse() {
            if !(self.matcher)(&element) {
                break;
            }
            result.push(element);
        }

        Ok(result)
    }
}

impl<T, P> Parsable<T> for Option<P>
where
    P: Parsable<T>,
    T: ConsumableToken,
{
    fn parser() -> impl Parser<T, Self> {
        OptionValidator::default()
    }
}

struct OptionValidator<P> {
    t: Matcher<P>,
}

impl<P> Default for OptionValidator<P> {
    fn default() -> Self {
        Self {
            t: Default::default(),
        }
    }
}

impl<T: ConsumableToken, P: Parsable<T>> Parser<T, Option<P>> for OptionValidator<P> {
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<Option<P>, ParseError> {
        let p = iter.parse()?;
        if (self.t)(&p) {
            Ok(Some(p))
        } else {
            Ok(None)
        }
    }
}

//
// #[cfg(test)]
// mod tests {
//
//     use crate::{
//         base_traits::Parsable, error::parse_error::ParseError, iter::TokenIter, t, token::Token,
//     };
//
//     #[derive(Debug, PartialEq, Clone)]
//     struct TestStruct {
//         ident: String,
//         semi: Option<Token>,
//     }
//
//     impl Parsable<Token> for TestStruct {
//         fn parse(iter: &mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
//         where
//             Self: Sized,
//         {
//             let ident= match iter.parse_if_match(|tok|matches!(tok, Token::Identifier(_)), None)?{
//                 Token::Identifier(string) => string,
//                 _ => unreachable!("Domain error: token returned by parse_if_match should be of the same variant as the token passed as argument"),
//             };
//             let semi = <Option<Token> as Parsable<Token>>::parse_if_match(
//                 iter,
//                 |tok| matches!(tok, Token::SemiColon),
//                 None,
//             )?;
//             Ok(TestStruct { ident, semi })
//         }
//     }
//
//     #[derive(Debug, PartialEq, Clone)]
//     struct VecStruct {
//         idents: Vec<Token>,
//     }
//
//     impl Parsable<Token> for VecStruct {
//         type ApplyMatchTo = Token;
//         fn parse(iter: &mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
//         where
//             Self: Sized,
//         {
//             let idents = iter.parse_if_match(|tok| matches!(tok, Token::Identifier(_)), None)?;
//             Ok(VecStruct { idents })
//         }
//         fn parse_if_match<F: Fn(&Token) -> bool>(
//             iter: &mut TokenIter<Token>,
//             f: F,
//             pattern: Option<&'static str>,
//         ) -> Result<Self, ParseError<Token>>
//         where
//             Self: Sized,
//         {
//             let idents = iter.parse_if_match(f, pattern)?;
//             Ok(VecStruct { idents })
//         }
//     }
//
//     #[derive(Debug, PartialEq, Clone)]
//     struct BoxStruct {
//         ident: Box<Token>,
//     }
//
//     impl Parsable<Token> for BoxStruct {
//         type ApplyMatchTo = Token;
//         fn parse(iter: &mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
//         where
//             Self: Sized,
//         {
//             let ident = Box::parse(iter)?;
//             Ok(BoxStruct { ident })
//         }
//
//         fn parse_if_match<F: Fn(&Token) -> bool>(
//             iter: &mut TokenIter<Token>,
//             f: F,
//             pattern: Option<&'static str>,
//         ) -> Result<Self, ParseError<Token>>
//         where
//             Self: Sized,
//         {
//             let ident = Box::parse_if_match(iter, f, pattern)?;
//             Ok(BoxStruct { ident })
//         }
//     }
//
//     #[test]
//     fn vec_of_tuples_arity2() {
//         let tokens = vec![t!(return), t!(litint 3), t!(return), t!(litint 3)];
//         let mut iter = TokenIter::new(tokens.clone());
//         let result: Vec<(Token, Token)> = iter.parse().unwrap();
//         assert_eq!(
//             result,
//             vec![
//                 (
//                     tokens.get(0).unwrap().clone(),
//                     tokens.get(1).unwrap().clone()
//                 ),
//                 (
//                     tokens.get(2).unwrap().clone(),
//                     tokens.get(3).unwrap().clone()
//                 )
//             ]
//         );
//
//         let tokens = vec![t!(return), t!(litint 3), t!(return)];
//         let mut iter = TokenIter::new(tokens.clone());
//         let result: Vec<(Token, Token)> = iter.parse().unwrap();
//         assert_eq!(
//             result,
//             vec![(
//                 tokens.get(0).unwrap().clone(),
//                 tokens.get(1).unwrap().clone()
//             ),]
//         );
//     }
//
//     #[test]
//     fn vec_of_tuples_arity3() {
//         let tokens = vec![
//             t!(return),
//             t!(litint 3),
//             t!(return),
//             t!(litint 3),
//             t!(return),
//             t!(litint 3),
//         ];
//         let mut iter = TokenIter::new(tokens.clone());
//         let result: Vec<(Token, Token, Token)> = iter.parse().unwrap();
//         assert_eq!(
//             result,
//             vec![
//                 (
//                     tokens.get(0).unwrap().clone(),
//                     tokens.get(1).unwrap().clone(),
//                     tokens.get(2).unwrap().clone(),
//                 ),
//                 (
//                     tokens.get(3).unwrap().clone(),
//                     tokens.get(4).unwrap().clone(),
//                     tokens.get(5).unwrap().clone(),
//                 )
//             ]
//         );
//
//         let tokens = vec![
//             t!(return),
//             t!(litint 3),
//             t!(return),
//             t!(litint 3),
//             t!(return),
//         ];
//         let mut iter = TokenIter::new(tokens.clone());
//         let result: Vec<(Token, Token, Token)> = iter.parse().unwrap();
//         assert_eq!(
//             result,
//             vec![(
//                 tokens.get(0).unwrap().clone(),
//                 tokens.get(1).unwrap().clone(),
//                 tokens.get(2).unwrap().clone(),
//             ),]
//         );
//     }
//     #[test]
//     fn box_ok() {
//         let tokens = vec![t!(ident "hello")];
//         let result = BoxStruct::parse(&mut TokenIter::new(tokens)).unwrap();
//         assert_eq!(
//             result,
//             BoxStruct {
//                 ident: Box::new(t!(ident "hello"))
//             }
//         );
//     }
//
//     #[test]
//     fn box_err() {
//         let tokens = vec![];
//         let result = BoxStruct::parse(&mut TokenIter::new(tokens));
//         assert!(result.is_err());
//     }
//
//     #[test]
//     fn box_match() {
//         let tokens = vec![t!(ident "hello")];
//         let result = BoxStruct::parse_if_match(
//             &mut TokenIter::new(tokens),
//             |t| matches!(t, Token::Identifier(_)),
//             None,
//         );
//         assert!(result.is_ok());
//     }
//
//     #[test]
//     fn vec_parse_if_match() {
//         let tokens = vec![t!(ident "ident1"), t!(ident "ident2")];
//
//         let result = VecStruct::parse_if_match(
//             &mut TokenIter::new(tokens),
//             |tok| matches!(tok, Token::Identifier(_)),
//             None,
//         )
//         .expect("Should be ok");
//         assert_eq!(
//             result.idents,
//             vec![
//                 Token::Identifier("ident1".to_owned()),
//                 Token::Identifier("ident2".to_owned())
//             ]
//         );
//     }
//
//     #[test]
//     fn parse_vec_with_many_elements() {
//         let tokens = vec![t!(ident "ident1"), t!(ident "ident2")];
//
//         let result = VecStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
//         assert_eq!(
//             result.idents,
//             vec![
//                 Token::Identifier("ident1".to_owned()),
//                 Token::Identifier("ident2".to_owned())
//             ]
//         );
//     }
//
//     #[test]
//     fn parse_vec_with_one_element() {
//         let tokens = vec![t!(ident "ident1")];
//
//         let result = VecStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
//         assert_eq!(result.idents, vec![Token::Identifier("ident1".to_owned())]);
//     }
//
//     #[test]
//     fn parse_vec_with_zero_elements() {
//         let tokens = vec![];
//
//         let result = VecStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
//         assert_eq!(result.idents, vec![]);
//     }
//
//     #[derive(Debug, Clone, PartialEq)]
//     struct ReturnStatement {
//         k_return: Option<Token>,
//         ident: Token,
//         semi: Option<Token>,
//     }
//
//     impl Parsable<Token> for ReturnStatement {
//         fn parse(iter: &mut TokenIter<Token>) -> Result<ReturnStatement, ParseError<Token>> {
//             let k_return = iter.parse_if_match(|input| matches!(input, Token::KReturn), None)?;
//             let ident = iter.parse_if_match(|input| matches!(input, Token::Identifier(_)), None)?;
//             let semi = iter.parse_if_match(|input| matches!(input, Token::SemiColon), None)?;
//             Ok(ReturnStatement {
//                 k_return,
//                 ident,
//                 semi,
//             })
//         }
//     }
//
//     #[test]
//     fn test1() {
//         let tokens = vec![t!(return), t!(ident "some_ident"), t!(;)];
//         let expected = ReturnStatement {
//             k_return: Some(t!(return)),
//             ident: t!(ident "some_ident"),
//             semi: Some(t!(;)),
//         };
//
//         let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
//         assert_eq!(Ok(expected), result);
//
//         let tokens = vec![t!(ident "some_ident"), t!(;)];
//         let expected = ReturnStatement {
//             k_return: None,
//             ident: t!(ident "some_ident"),
//             semi: Some(t!(;)),
//         };
//
//         let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
//         assert_eq!(Ok(expected), result);
//
//         let tokens = vec![t!(return), t!(ident "some_ident")];
//
//         let expected = ReturnStatement {
//             k_return: Some(t!(return)),
//             ident: t!(ident "some_ident"),
//             semi: None,
//         };
//
//         let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
//         assert_eq!(Ok(expected), result);
//     }
//
//     #[test]
//     fn parse_option_none() {
//         let tokens = vec![t!(ident "ident1")];
//
//         let result = TestStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
//         assert_eq!(result.ident, "ident1");
//         assert!(result.semi.is_none());
//     }
//
//     #[test]
//     fn parse_option_some() {
//         let tokens = vec![t!(ident "ident1"), t!(;)];
//
//         let result = TestStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
//         assert!(result.ident == "ident1");
//         assert!(result.semi == Some(Token::SemiColon));
//     }
// }
