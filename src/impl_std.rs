use crate::{
    base_traits::Parsable, error::parse_error::ParseError, iter::TokenIter, ConsumableToken,
    Expectable,
};

impl<T, P> Parsable<T> for Vec<P>
where
    P: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>> {
        let mut results = vec![];
        while let Ok(r) = P::parse(iter) {
            results.push(r);
        }
        Ok(results)
    }
}

impl<T, P> Parsable<T> for Option<P>
where
    P: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>> {
        let r = P::parse(iter);
        match r {
            Ok(r) => Ok(Some(r)),
            Err(_) => unimplemented!("Error types are necessary here"),
        }
    }
}

impl<T> Expectable<T> for Option<T>
where
    T: ConsumableToken,
{
    fn expect(iter: &mut TokenIter<T>, expected_token: T) -> Result<Self, ParseError<T>> {
        match iter.get(iter.current) {
            Some(found_token) if found_token.stateless_equals(&expected_token) => Ok(Some(expected_token)),
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        base_traits::Parsable, error::parse_error::ParseError, iter::TokenIter, t, token::Token,
        Expectable,
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
            let ident = match iter.expect(|tok|matches!(tok, Token::Identifier(_)))?{
                Token::Identifier(string) => string,
                _ => unreachable!("Domain error: token returned by expect should be of the same variant as the token passed as argument"),
            };
            let semi = <Option<Token> as Expectable<Token>>::expect(iter, Token::SemiColon)?;
            Ok(TestStruct { ident, semi })
        }
    }

    #[test]
    fn parse_option_none() {
        let tokens = vec![t!(ident "ident1")];

        let result = TestStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
        assert_eq!(result.ident,"ident1");
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