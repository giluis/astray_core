use super::*;

impl<T, P> Parsable<T> for Option<P>
where
    P: Parsable<T>,
    T: ConsumableToken,
{
    type P = OptionValidator<P>;
    fn parser() -> Self::P{
        OptionValidator::default()
    }
}

pub struct OptionValidator<P> {
    t: Pattern<P>,
}

impl<P> Default for OptionValidator<P> {
    fn default() -> Self {
        Self {
            t: Default::default(),
        }
    }
}

impl <P> OptionValidator<P> {
    pub fn with_matcher(&mut self, m: Pattern<P>) -> &mut Self {
        self.t = m ;
        self
    }
}

impl<T: ConsumableToken, P: Parsable<T>> Parser<T, Option<P>> for OptionValidator<P> {
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<Option<P>, ParseError> {
        iter.try_do(|token_iter| {
            match token_iter.parse() {
                Ok(p) if (self.t)(&p) => Ok(Some(p)),
                Ok(p) => Ok(None),
                Err(err) => {dbg!(err);Ok(None)},
            } 
        })
    }
}


#[cfg(test)]
mod tests {
    use derive_builder::Builder;
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct TestStruct {
        ident: String,
        semi: Option<Token>,
    }

    impl Parsable<Token> for TestStruct {
        type P = TestStructParser;
        fn parser() -> Self::P{
            TestStructParser::default()
        }
    }

    #[derive(Default, Builder)]
    struct TestStructParser {
        ident: Pattern<String>,
        semi: Pattern<Token>,
    }

    impl Parser<Token, TestStruct> for TestStructParser {
        fn parse(&self, iter: &mut TokenIter<Token>) -> Result<TestStruct, ParseError> {
            iter.try_do(|token_iter| {

            let ident = match Token::parser().with_matcher(matcher!(Token::Identifier(_))).parse(token_iter)?{
                Token::Identifier(string) => string,
                _ => unreachable!("Domain error: token returned by parse_if_match should be of the same variant as the token passed as argument"),
            };
            let semi = <Option::<Token> as Parsable<Token>>::parser().with_matcher(matcher!(t!(;))).parse(token_iter).unwrap();
            // .with_matcher(matcher!(t!(;))).parse(token_iter)?;
            
            // .with_matcher(matcher!(t!(;))).parse(token_iter)

            Ok(TestStruct { ident, semi })
            })
        }
    }

    #[test]
    fn option_none_when_unmatching() {
        let mut tokens = TokenIter::new(vec![t!(ident "ident1")]);
        let result = Option::<Token>::parser().with_matcher(
            Pattern{
                fun: |t| {match t {
                Token::Identifier(placeholder) if matches!(placeholder.as_ref(), "NOT_IDENT_1") => true,
                Token::Identifier(_) => false,
                _ => unreachable!("Internal error here")
            }},
            pat: "NOT_IDENT_1"
         }).parse(&mut tokens);

        assert!(result.is_ok());
        assert!(result.unwrap().is_none())
    }

    #[test]
    fn option_is_none_when_cannot_parse() {
        let mut tokens = TokenIter::new(vec![t!(;)]);
        let result = Option::<Token>::parser().with_matcher(matcher!(Token::Identifier(_))).parse(&mut tokens);

        assert!(result.is_ok());
        assert!(result.unwrap().is_none())
    }

    #[test]
    fn option_is_some_when_when_parsed_and_matched() {
        let mut tokens = TokenIter::new(vec![t!(;)]);
        let result = Option::<Token>::parser().with_matcher(matcher!(t!(;))).parse(&mut tokens);

        assert!(result.is_ok());
        assert!(result.unwrap().is_some())
    }

}