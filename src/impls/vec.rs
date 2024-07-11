use super::*;

impl<T, P> Parsable<T> for Vec<P>
where
    P: Parsable<T>,
    T: ConsumableToken,
{
    type P = VecValidator<P>;
    
    fn parser() -> Self::P{
        VecValidator::default()
    }
}

pub struct VecValidator<P> {
    matcher: Pattern<P>,
}

impl<P> Default for VecValidator<P> {
    fn default() -> Self {
        Self {
            matcher: Default::default(),
        }
    }
}

impl<P> VecValidator<P> {
    pub fn with_matcher(&mut self, m: &Pattern<P>) -> &mut Self {
        self.matcher = m.clone();
        self
    }
}

impl<T: ConsumableToken, P: Parsable<T>> Parser<T, Vec<P>> for VecValidator<P> {
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<Vec<P>, ParseError> {
        let mut result = vec![];

        while let Ok(element) = iter.try_do(|token_iter| {
            let parsed = token_iter.parse()?;

            if (self.matcher)(&parsed) {
                Ok(parsed)
            } else {
                Err(ParseError::parsed_but_unmatching(
                    token_iter.current,
                    &parsed,
                    "pattern hello",
                ))
            }
        }) {
            result.push(element)
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        matcher, t, ConsumableToken, Pattern, Parsable, ParseError, Parser, Token, TokenIter,
    };

    #[derive(Debug, PartialEq, Clone)]
    struct VecStruct {
        fn_name: Token,
        idents: Vec<Token>,
    }

    #[derive(Default)]
    struct VecStructParser {
        m: Pattern<Token>,
    }

    impl Parsable<Token> for VecStruct {
        type P= VecStructParser;
        fn parser() -> Self::P {
            VecStructParser::default()
        }
    }

    impl VecStructParser {
        fn with_matcher(&mut self, m: &Pattern<Token>) -> &mut Self {
            self.m = m.clone();
            self
        }
    }

    impl Parser<Token, VecStruct> for VecStructParser {
        fn parse(&self, iter: &mut TokenIter<Token>) -> Result<VecStruct, ParseError>
        where
            Self: Sized,
        {
            let fn_name = Token::parser()
                .with_matcher(matcher!(Token::LiteralInt(_)))
                .parse(iter)?;
            let idents = Vec::<Token>::parser().with_matcher(&self.m).parse(iter)?;
            Ok(VecStruct { fn_name, idents })
        }
    }

    #[test]
    fn vec_parse_2() {
        let litint = t!(litint 4);
        let mut idents = vec![t!(ident "ident1"), t!(ident "ident2")];
        let mut tokens = vec![litint];
        tokens.append(&mut idents);
        let mut iter = TokenIter::new(tokens);

        let result = VecStruct::parser()
            .with_matcher(&matcher!(Token::Identifier(_)))
            .parse(&mut iter)
            .expect("Expected Ok Result");

        assert_eq!(
            result.idents,
            vec![
                Token::Identifier("ident1".to_owned()),
                Token::Identifier("ident2".to_owned())
            ]
        );
        assert_eq!(result.fn_name, t!(litint 4));

        assert!(iter.is_at_end())
    }

    #[test]
    fn vec_parse_1() {
        let litint = t!(litint 4);
        let mut idents = vec![t!(ident "ident1")];
        let mut tokens = vec![litint.clone()];
        tokens.append(&mut idents);
        let mut iter = TokenIter::new(tokens);

        let result = VecStruct::parser()
            .with_matcher(&matcher!(Token::Identifier(_)))
            .parse(&mut iter)
            .expect("Expected Ok Result");

        assert_eq!(result.idents, vec![Token::Identifier("ident1".to_owned()),]);
        assert_eq!(result.fn_name, litint);
        assert!(iter.is_at_end())
    }

    #[test]
    fn vec_parse_0() {
        let litint = t!(litint 4);
        let mut idents = vec![];
        let mut tokens = vec![litint.clone()];
        tokens.append(&mut idents);
        let mut iter = TokenIter::new(tokens);

        let result = VecStruct::parser()
            .with_matcher(&matcher!(Token::Identifier(_)))
            .parse(&mut iter)
            .expect("Expected Ok Result");

        assert_eq!(result.idents, vec![]);
        assert_eq!(result.fn_name, litint);
        assert!(iter.is_at_end())
    }

    #[test]
    fn vec_leaves_undesirables() {
        let litint = t!(litint 4);
        let mut idents = vec![t!(ident "ident1"), t!(ident "ident2")];
        let mut tokens = vec![litint.clone()];
        tokens.append(&mut idents);
        tokens.push(t!(;));
        let mut iter = TokenIter::new(tokens);

        let result = VecStruct::parser()
            .with_matcher(&matcher!(Token::Identifier(_)))
            .parse(&mut iter)
            .expect("Expected Ok Result");

        assert_eq!(result.idents, vec![t!(ident "ident1"), t!(ident "ident2")]);
        assert_eq!(result.fn_name, litint);
        assert!(iter.is_at_end() == false);
        let _ = iter.consume();
        assert!(iter.is_at_end())
    }
}
