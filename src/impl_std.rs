use crate::{
    base_traits::{Pattern, Parsable, Parser},
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
    type P = Tuple2Validator<P1, P2>;

    fn parser() -> Self::P{
        Tuple2Validator::default()
    }
}

pub struct Tuple2Validator<P1, P2>(Pattern<P1>, Pattern<P2>);

impl<P1, P2> Default for Tuple2Validator<P1, P2> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl <P1, P2> Tuple2Validator<P1, P2> {
    pub fn with_matchers(&mut self, p0: Pattern<P1>, p1: Pattern<P2>) -> &mut Self{
        self.0 = p0;
        self.1 = p1;
        self
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
    type P = Tuple3Validator<P1, P2, P3>;
    fn parser() -> Self::P{
        Tuple3Validator::default()
    }
}

pub struct Tuple3Validator<P1, P2, P3>(Pattern<P1>, Pattern<P2>, Pattern<P3>);

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
    type P = Tuple4Validator<P1, P2, P3, P4>;
    fn parser() -> Self::P
    where
        Self: Sized,
    {
        Tuple4Validator::default()
    }
}

pub struct Tuple4Validator<P1, P2, P3, P4>(Pattern<P1>, Pattern<P2>, Pattern<P3>, Pattern<P4>);

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



#[cfg(test)]
mod tests {
    use crate::Token;




    // #[test]
    // fn vec_of_tuples_arity3() {
    //     let tokens = vec![
    //         t!(return),
    //         t!(litint 3),
    //         t!(return),
    //         t!(litint 3),
    //         t!(return),
    //         t!(litint 3),
    //     ];
    //     let mut iter = TokenIter::new(tokens.clone());
    //     let result: Vec<(Token, Token, Token)> = iter.parse().unwrap();
    //     assert_eq!(
    //         result,
    //         vec![
    //             (
    //                 tokens.get(0).unwrap().clone(),
    //                 tokens.get(1).unwrap().clone(),
    //                 tokens.get(2).unwrap().clone(),
    //             ),
    //             (
    //                 tokens.get(3).unwrap().clone(),
    //                 tokens.get(4).unwrap().clone(),
    //                 tokens.get(5).unwrap().clone(),
    //             )
    //         ]
    //     );

    //     let tokens = vec![
    //         t!(return),
    //         t!(litint 3),
    //         t!(return),
    //         t!(litint 3),
    //         t!(return),
    //     ];
    //     let mut iter = TokenIter::new(tokens.clone());
    //     let result: Vec<(Token, Token, Token)> = iter.parse().unwrap();
    //     assert_eq!(
    //         result,
    //         vec![(
    //             tokens.get(0).unwrap().clone(),
    //             tokens.get(1).unwrap().clone(),
    //             tokens.get(2).unwrap().clone(),
    //         ),]
    //     );
    // }


    // #[derive(Debug, Clone, PartialEq)]
    // struct ReturnStatement {
    //     k_return: Option<Token>,
    //     ident: Token,
    //     semi: Option<Token>,
    // }

    // impl Parsable<Token> for ReturnStatement {
    //     fn parse(iter: &mut TokenIter<Token>) -> Result<ReturnStatement, ParseError<Token>> {
    //         let k_return = iter.parse_if_match(|input| matches!(input, Token::KReturn), None)?;
    //         let ident = iter.parse_if_match(|input| matches!(input, Token::Identifier(_)), None)?;
    //         let semi = iter.parse_if_match(|input| matches!(input, Token::SemiColon), None)?;
    //         Ok(ReturnStatement {
    //             k_return,
    //             ident,
    //             semi,
    //         })
    //     }
    // }

    // #[test]
    // fn test1() {
    //     let tokens = vec![t!(return), t!(ident "some_ident"), t!(;)];
    //     let expected = ReturnStatement {
    //         k_return: Some(t!(return)),
    //         ident: t!(ident "some_ident"),
    //         semi: Some(t!(;)),
    //     };

    //     let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
    //     assert_eq!(Ok(expected), result);

    //     let tokens = vec![t!(ident "some_ident"), t!(;)];
    //     let expected = ReturnStatement {
    //         k_return: None,
    //         ident: t!(ident "some_ident"),
    //         semi: Some(t!(;)),
    //     };

    //     let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
    //     assert_eq!(Ok(expected), result);

    //     let tokens = vec![t!(return), t!(ident "some_ident")];

    //     let expected = ReturnStatement {
    //         k_return: Some(t!(return)),
    //         ident: t!(ident "some_ident"),
    //         semi: None,
    //     };

    //     let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
    //     assert_eq!(Ok(expected), result);
    // }

}
