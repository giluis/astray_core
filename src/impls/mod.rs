use crate::{matcher, t, base_traits::{Parsable, ConsumableToken, Parser, Pattern}, iter::TokenIter, error::ParseError, token::{Token}} ;

mod vec;
mod option;
mod r#box;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_of_tuples_arity2() {
        let tokens = vec![t!(return), t!(litint 3), t!(return), t!(litint 3)];
        let mut iter = TokenIter::new(tokens.clone());
        let mut parser = Vec::<(Token,Token)>::parser();
        parser.with_matcher(&matcher!((Token::KReturn, Token::LiteralInt(_))));
        let result = parser.parse(&mut iter).expect("Should have parsed Vec with no issue");
        dbg!(&result);
        // 2 tuples of len == 4
        assert!(result.len() == 2);
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

        assert!(iter.is_at_end());

        let tokens = vec![t!(return), t!(litint 3), t!(return)];
        let mut iter = TokenIter::new(tokens.clone());
        let result = parser.parse(&mut iter).expect("Should have parsed Vec with no issue");
        assert!(result.len() == 1);
        assert_eq!(
            result,
            vec![(
                tokens.get(0).unwrap().clone(),
                tokens.get(1).unwrap().clone()
            ),]
        );
        assert!(iter.current == 2);
    }

}