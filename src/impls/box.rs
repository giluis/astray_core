// TODO: macro for tuple implementation
use super::*;

impl<T, P> Parsable<T> for Box<P>
where
    P: Parsable<T>,
    T: ConsumableToken,
{
    type P = BoxValidator<P>;
    fn parser() -> Self::P
    where
        Self: Sized,
    {
        BoxValidator::default()
    }
}

pub struct BoxValidator<P> {
    m: Pattern<P>,
}

impl <P> BoxValidator<P> {
    pub fn with_matcher(&mut self, m: &Pattern<P>) ->  &mut Self {
        self.m = m.clone();
        self
    }

}

impl<P> Default for BoxValidator<P> {
    fn default() -> Self {
        Self {
            m: Default::default(),
        }
    }
}

impl<T: ConsumableToken, P: Parsable<T>> Parser<T, Box<P>> for BoxValidator<P> {
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<Box<P>, ParseError> {
        iter.try_do(|token_iter| {
            let p = token_iter.parse::<P>()?;
            dbg!(&self.m);
            if (self.m)(&p) {
                Ok(Box::new(p))
            } else {
                Err(ParseError::parsed_but_unmatching(
                    token_iter.current,
                    &p,
                    // TODO: add nice error message
                    self.m.pat
                ))
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    struct BoxStruct {
        ident: Box<Token>,
    }

    impl Parsable<Token> for BoxStruct {
        type P = BoxStructParser;
        fn parser() -> Self::P {
            BoxStructParser::default()
        }
    }

    #[derive(Default)]
    struct BoxStructParser {
        m: Pattern<Token>
    }

    impl BoxStructParser {
        fn with_matcher(&mut self, m: Pattern<Token>) -> &mut Self {
            self.m = m.clone();
            self
        }

    }


    impl Parser<Token, BoxStruct> for BoxStructParser {
        fn parse(&self, iter: &mut TokenIter<Token>) -> Result<BoxStruct, ParseError>
        where
            Self: Sized,
        {
            let ident = Box::<Token>::parser().with_matcher(&self.m).parse(iter)?;
            Ok(BoxStruct { ident })
        }
    }


    #[test]
    fn different_patterns() {
        let litint1 = t!(litint 4);
        let litint2 = t!(litint 6);

        let mut tokens = TokenIter::new(vec![litint1.clone(), litint2.clone()]);

        let result = BoxStruct::parser().with_matcher(matcher!(Token::LiteralInt(_))).parse(&mut tokens).expect("Expected Ok Result");
        assert_eq!(result, BoxStruct{ident: Box::new(litint1)});

        let result = BoxStruct::parser().with_matcher(matcher!(Token::LiteralInt(_))).parse(&mut tokens).expect("Expected Ok Result");
        assert_eq!(result, BoxStruct{ident: Box::new(litint2)});

        assert!(tokens.is_at_end());

        let result = BoxStruct::parser().with_matcher(matcher!(Token::LiteralInt(_))).parse(&mut tokens);
        assert_eq!(result, Err(ParseError::no_more_tokens::<Token>(2)));
    }

    #[test]
    fn empty_test() {

        let tokens = vec![];
        let result = BoxStruct::parser().parse(&mut TokenIter::new(tokens));
        assert!(result.is_err());
    }
}
