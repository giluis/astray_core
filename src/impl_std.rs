use crate::{base_traits::Parsable, error::parse_error::ParseError, iter::TokenIter, ConsumableToken};

impl<P1, P2, T> Parsable<T> for (P1, P2)
where
    P1: Parsable<T>,
    P2: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        Ok((iter.parse()?, iter.parse()?))
    }

    fn parse_if_match<F: Fn(&Self::ApplyMatchTo) -> bool>(
        iter: &mut TokenIter<T>,
        matches: F,
        pattern: Option<&'static str>
    ) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        iter.try_do(|token_iter| {
            let result = Self::parse(token_iter)?;
            if matches(&result) {
                Ok(result)
            } else {
                Err(ParseError::parsed_but_unmatching::<Self>(token_iter.current, &result,pattern))
            }
        })
    }
}

impl<P1, P2, P3, T> Parsable<T> for (P1, P2, P3)
where
    P1: Parsable<T>,
    P2: Parsable<T>,
    P3: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        Ok((
            iter.parse::<P1>()?,
            iter.parse::<P2>()?,
            iter.parse::<P3>()?,
        ))
    }

    fn parse_if_match<F: Fn(&Self) -> bool>(
        iter: &mut TokenIter<T>,
        matches: F,
        pattern: Option<&'static str>
    ) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        iter.try_do(|token_iter| {
            let result = Self::parse(token_iter)?;
            if matches(&result) {
                Ok(result)
            } else {
                Err(ParseError::parsed_but_unmatching::<Self>(token_iter.current, &result,pattern))
            }
        })
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
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        Ok((
            iter.parse::<P1>()?,
            iter.parse::<P2>()?,
            iter.parse::<P3>()?,
            iter.parse::<P4>()?,
        ))
    }
}


// TODO: macro for tuple implementation

impl<T, P> Parsable<T> for Box<P>
where
    P: Parsable<T, ApplyMatchTo = P>,
    T: ConsumableToken,
{
    type ApplyMatchTo = P;
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        Ok(Box::new(P::parse(iter)?))
    }

    fn parse_if_match<F: Fn(&Self::ApplyMatchTo) -> bool>(
        iter: &mut TokenIter<T>,
        matches: F,
        pattern: Option<&'static str>
    ) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        Ok(Box::new(iter.parse_if_match(matches, pattern)?))
    }
}

impl<T, P> Parsable<T> for Vec<P>
where
    P: Parsable<T, ApplyMatchTo = P>,
    T: ConsumableToken,
{
    type ApplyMatchTo = P;
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>> {
        let mut results = vec![];
        while let Ok(r) = P::parse(iter) {
            results.push(r);
        }
        Ok(results)
    }

    fn parse_if_match<F>(iter: &mut TokenIter<T>, matches: F, pattern: Option<&'static str>) -> Result<Vec<P>, ParseError<T>>
    where
        F: Fn(&Self::ApplyMatchTo) -> bool,
    {
        let mut result = vec![];
        while let Ok(element) = Self::ApplyMatchTo::parse_if_match(iter, &matches, pattern) {
            result.push(element)
        }

        Ok(result)
    }
}

impl<T, P> Parsable<T> for Option<P>
where
    P: Parsable<T, ApplyMatchTo = P>,
    T: ConsumableToken,
{
    type ApplyMatchTo = P;
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>> {
        let r = iter.parse();
        match r {
            Ok(r) => Ok(Some(r)),
            Err(_) => Ok(None),
        }
    }

    fn parse_if_match<F: Fn(&Self::ApplyMatchTo) -> bool>(
        iter: &mut TokenIter<T>,
        matches: F,
        pattern: Option<&'static str>
    ) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        let r: Result<Self::ApplyMatchTo, _> = iter.parse_if_match(matches, pattern);
        match r {
            Ok(r) => Ok(Some(r)),
            Err(_) => Ok(None),
        }
    }
}
pub trait IfOk<T, E> {
    fn if_ok(self, value: T) -> Result<T, E>;
}

impl<P, T, E> IfOk<T, E> for Result<P, E> {
    fn if_ok(self, value: T) -> Result<T, E> {
        match self {
            Ok(_) => Ok(value),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        base_traits::Parsable, error::parse_error::ParseError, iter::TokenIter, t, token::Token,
    };

    #[derive(Debug, PartialEq, Clone)]
    struct TestStruct {
        ident: String,
        semi: Option<Token>,
    }

    impl Parsable<Token> for TestStruct {
        fn parse(iter: &mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
        where
            Self: Sized,
        {
            let ident= match iter.parse_if_match(|tok|matches!(tok, Token::Identifier(_)), None)?{
                Token::Identifier(string) => string,
                _ => unreachable!("Domain error: token returned by parse_if_match should be of the same variant as the token passed as argument"),
            };
            let semi = <Option<Token> as Parsable<Token>>::parse_if_match(iter, |tok| {
                matches!(tok, Token::SemiColon)
            }, None)?;
            Ok(TestStruct { ident, semi })
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    struct VecStruct {
        idents: Vec<Token>,
    }

    impl Parsable<Token> for VecStruct {
        type ApplyMatchTo = Token;
        fn parse(iter: &mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
        where
            Self: Sized,
        {
            let idents = iter.parse_if_match(|tok| matches!(tok, Token::Identifier(_)), None)?;
            Ok(VecStruct { idents })
        }
        fn parse_if_match<F: Fn(&Token) -> bool>(
            iter: &mut TokenIter<Token>,
            f: F,
            pattern: Option<&'static str>
        ) -> Result<Self, ParseError<Token>>
        where
            Self: Sized,
        {
            let idents = iter.parse_if_match(f, pattern)?;
            Ok(VecStruct { idents })
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    struct BoxStruct {
        ident: Box<Token>,
    }

    impl Parsable<Token> for BoxStruct {
        type ApplyMatchTo = Token;
        fn parse(iter: &mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
        where
            Self: Sized,
        {
            let ident = Box::parse(iter)?;
            Ok(BoxStruct { ident })
        }

        fn parse_if_match<F: Fn(&Token) -> bool>(
            iter: &mut TokenIter<Token>,
            f: F,
            pattern: Option<&'static str>
        ) -> Result<Self, ParseError<Token>>
        where
            Self: Sized,
        {
            let ident = Box::parse_if_match(iter, f, pattern)?;
            Ok(BoxStruct { ident })
        }
    }

    #[test]
    fn vec_of_tuples_arity2() {
        let tokens = vec![t!(return), t!(litint 3), t!(return), t!(litint 3)];
        let mut iter = TokenIter::new(tokens.clone());
        let result: Vec<(Token, Token)> = iter.parse().unwrap();
        assert_eq!(
            result,
            vec![
                (
                    tokens.get(0).unwrap().clone(),
                    tokens.get(1).unwrap().clone()
                ),
                (
                    tokens.get(2).unwrap().clone(),
                    tokens.get(3).unwrap().clone()
                )
            ]
        );

        let tokens = vec![t!(return), t!(litint 3), t!(return)];
        let mut iter = TokenIter::new(tokens.clone());
        let result: Vec<(Token, Token)> = iter.parse().unwrap();
        assert_eq!(
            result,
            vec![(
                tokens.get(0).unwrap().clone(),
                tokens.get(1).unwrap().clone()
            ),]
        );
    }

    #[test]
    fn vec_of_tuples_arity3() {
        let tokens = vec![
            t!(return),
            t!(litint 3),
            t!(return),
            t!(litint 3),
            t!(return),
            t!(litint 3),
        ];
        let mut iter = TokenIter::new(tokens.clone());
        let result: Vec<(Token, Token, Token)> = iter.parse().unwrap();
        assert_eq!(
            result,
            vec![
                (
                    tokens.get(0).unwrap().clone(),
                    tokens.get(1).unwrap().clone(),
                    tokens.get(2).unwrap().clone(),
                ),
                (
                    tokens.get(3).unwrap().clone(),
                    tokens.get(4).unwrap().clone(),
                    tokens.get(5).unwrap().clone(),
                )
            ]
        );

        let tokens = vec![
            t!(return),
            t!(litint 3),
            t!(return),
            t!(litint 3),
            t!(return),
        ];
        let mut iter = TokenIter::new(tokens.clone());
        let result: Vec<(Token, Token, Token)> = iter.parse().unwrap();
        assert_eq!(
            result,
            vec![(
                tokens.get(0).unwrap().clone(),
                tokens.get(1).unwrap().clone(),
                tokens.get(2).unwrap().clone(),
            ),]
        );
    }
    #[test]
    fn box_ok() {
        let tokens = vec![t!(ident "hello")];
        let result = BoxStruct::parse(&mut TokenIter::new(tokens)).unwrap();
        assert_eq!(
            result,
            BoxStruct {
                ident: Box::new(t!(ident "hello"))
            }
        );
    }

    #[test]
    fn box_err() {
        let tokens = vec![];
        let result = BoxStruct::parse(&mut TokenIter::new(tokens));
        assert!(result.is_err());
    }

    #[test]
    fn box_match() {
        let tokens = vec![t!(ident "hello")];
        let result = BoxStruct::parse_if_match(&mut TokenIter::new(tokens), |t| {
            matches!(t, Token::Identifier(_))
        }, None);
        assert!(result.is_ok());
    }

    #[test]
    fn vec_parse_if_match() {
        let tokens = vec![t!(ident "ident1"), t!(ident "ident2")];

        let result = VecStruct::parse_if_match(&mut TokenIter::new(tokens), |tok| {
            matches!(tok, Token::Identifier(_))
        }, None)
        .expect("Should be ok");
        assert_eq!(
            result.idents,
            vec![
                Token::Identifier("ident1".to_owned()),
                Token::Identifier("ident2".to_owned())
            ]
        );
    }

    #[test]
    fn parse_vec_with_many_elements() {
        let tokens = vec![t!(ident "ident1"), t!(ident "ident2")];

        let result = VecStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
        assert_eq!(
            result.idents,
            vec![
                Token::Identifier("ident1".to_owned()),
                Token::Identifier("ident2".to_owned())
            ]
        );
    }

    #[test]
    fn parse_vec_with_one_element() {
        let tokens = vec![t!(ident "ident1")];

        let result = VecStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
        assert_eq!(result.idents, vec![Token::Identifier("ident1".to_owned())]);
    }

    #[test]
    fn parse_vec_with_zero_elements() {
        let tokens = vec![];

        let result = VecStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
        assert_eq!(result.idents, vec![]);
    }

    #[derive(Debug, Clone, PartialEq)]
    struct ReturnStatement {
        k_return: Option<Token>,
        ident: Token,
        semi: Option<Token>,
    }

    impl Parsable<Token> for ReturnStatement {
        fn parse(iter: &mut TokenIter<Token>) -> Result<ReturnStatement, ParseError<Token>> {
            let k_return = iter.parse_if_match(|input| matches!(input, Token::KReturn), None)?;
            let ident = iter.parse_if_match(|input| matches!(input, Token::Identifier(_)), None)?;
            let semi = iter.parse_if_match(|input| matches!(input, Token::SemiColon), None)?;
            Ok(ReturnStatement {
                k_return,
                ident,
                semi,
            })
        }
    }

    #[test]
    fn test1() {
        let tokens = vec![t!(return), t!(ident "some_ident"), t!(;)];
        let expected = ReturnStatement {
            k_return: Some(t!(return)),
            ident: t!(ident "some_ident"),
            semi: Some(t!(;)),
        };

        let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
        assert_eq!(Ok(expected), result);

        let tokens = vec![t!(ident "some_ident"), t!(;)];
        let expected = ReturnStatement {
            k_return: None,
            ident: t!(ident "some_ident"),
            semi: Some(t!(;)),
        };

        let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
        assert_eq!(Ok(expected), result);

        let tokens = vec![t!(return), t!(ident "some_ident")];

        let expected = ReturnStatement {
            k_return: Some(t!(return)),
            ident: t!(ident "some_ident"),
            semi: None,
        };

        let result = ReturnStatement::parse(&mut TokenIter::new(tokens));
        assert_eq!(Ok(expected), result);
    }

    #[test]
    fn parse_option_none() {
        let tokens = vec![t!(ident "ident1")];

        let result = TestStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
        assert_eq!(result.ident, "ident1");
        assert!(result.semi.is_none());
    }

    #[test]
    fn parse_option_some() {
        let tokens = vec![t!(ident "ident1"), t!(;)];

        let result = TestStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
        assert!(result.ident == "ident1");
        assert!(result.semi == Some(Token::SemiColon));
    }
}
