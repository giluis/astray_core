use crate::{base_traits::Parsable, error::parse_error::ParseError, ConsumableToken};

pub struct TokenIter<Token> {
    pub current: usize,
    pub tokens: Vec<Token>,
    size: usize,
    pub stack: Vec<usize>,
}

impl<Token> TokenIter<Token>
where
    Token: ConsumableToken,
{
    pub fn new(tokens: Vec<Token>) -> TokenIter<Token> {
        TokenIter {
            current: 0,
            size: tokens.len(),
            tokens,
            stack: vec![],
        }
    }

    pub fn parse<P>(&mut self) -> Result<P, ParseError<Token>>
    where
        P: Parsable<Token>,
    {
        self.push();
        let result = P::parse(self);
        self.clean_pop();
        // if result.is_ok() {
        //     self.clean_pop();
        // } else {
        //     self.pop();
        // }
        result
    }

    pub fn try_do<F, Q, E>(&mut self, f: F) -> Result<Q, E>
    where
        F: FnOnce(&mut TokenIter<Token>) -> Result<Q, E>,
    {
        self.push();
        let result = f(self);
        if result.is_ok() {
            self.clean_pop();
        } else {
            self.pop();
        }
        result
    }

    pub fn push(&mut self) {
        self.stack.push(self.current);
    }

    pub fn clean_pop(&mut self) {
        self.stack.pop();
    }

    pub fn pop(&mut self) -> Option<usize> {
        match self.stack.pop() {
            Some(c) => {
                self.current = c;
                Some(c)
            }
            None => None,
        }
    }

    pub fn consume(&mut self) -> Option<Token> {
        match self.get(self.current) {
            Some(element) => {
                self.current += 1;
                Some(element)
            }
            None => None,
        }
    }

    pub fn get(&self, position: usize) -> Option<Token> {
        if position < self.size {
            Some(self.tokens[position].clone())
        } else {
            None
        }
    }


    pub fn expect<F>(&mut self, matches:F) -> Result<Token,ParseError<Token>> 
    where F: FnOnce(&Token) -> bool  {
        self.try_do(|token_iter|{
            match token_iter.consume() {
                Some(ref found) if matches(found) => Ok(found.clone()),
                Some(ref found) => Err(ParseError::unmatching_token(token_iter.current, "Failed to expected token not found".to_string(), found.clone())),
                _ => Err(ParseError::no_more_tokens(token_iter.current))
            }
        })
    }
    // TODO: this clone is expensive in the long run
    // I must find a way to prevent it from happening
    pub fn expect_msg<F>(&mut self, matches:F, msg: String) -> Result<Token,ParseError<Token>> 
    where F: FnOnce(&Token) -> bool  {
        self.try_do(|token_iter|{
            match token_iter.consume() {
                Some(ref found) if matches(found) => Ok(found.clone()),
                Some(ref found) => Err(ParseError::unmatching_token(token_iter.current, msg, found.clone())),
                _ => Err(ParseError::no_more_tokens(token_iter.current))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_common::TestStruct;
    use crate::{t, Parsable, Token, TokenIter, ParseError};

    #[test]
    fn expect_match_enum_token() {
        let mut iter: TokenIter<Token> = TokenIter::new(vec![
            Token::Comma,
            Token::Identifier("Some identifier".to_string()),
        ]);

        let result = iter.expect(|t| matches!(t,Token::Comma)); 
        assert_eq!(result,Ok(Token::Comma));

        let result = iter.expect(|t| matches!(t, Token::Identifier(_)));
        assert_eq!(result,Ok(Token::Identifier("Some identifier".to_string())));

        let result = iter.expect(|t| matches!(t, Token::Identifier(_)));
        assert_eq!(result, Err(ParseError::no_more_tokens(2)));

        let mut iter: TokenIter<Token> = TokenIter::new(vec![
            Token::Identifier("Some identifier".to_string()),
        ]);

        let result = iter.expect(|t| matches!(t, Token::Comma));
        assert!(result.is_err());
    }

    //TODO: check error is the correct one
    #[test]
    fn failed_expect() {
        let tokens = vec![t!(litint 32)];
        let mut iter = TokenIter::new(tokens);
        let result = iter.expect(|tok|matches!(tok, t!(return)));
        assert!(result.is_err());
        assert!(iter.current == 0);
    }

    #[test]
    fn failed_parse() {
        let tokens = vec![
            t!(int),
            // variable_name is missing, so TestStruct will not be parsed
            t!( = ),
            t!(litint 3),
        ];

        let mut iter = TokenIter::new(tokens);
        let result = TestStruct::parse(&mut iter);
        assert!(result.is_err());

        // current should be zero, since struct was not parsed
        assert!(iter.current == 0)
    }

    #[test]
    fn successful_parse() {
        let expected_var_name = "variable1";
        let expected_value = 3;
        let tokens = vec![
            t!(int),
            t!(ident expected_var_name),
            t!( = ),
            t!(litint expected_value),
            Token::LiteralInt(expected_value),
            t!( ; ),
        ];
        let mut iter = TokenIter::new(tokens);
        let expected_struct = TestStruct {
            var_type: t!(int),
            var_name: expected_var_name.to_string(),
            equals_sign: t!( = ),
            value: expected_value,
        };
        let result = TestStruct::parse(&mut iter)
            .expect("Should succeed, since tokens represent a valid TestStruct");
        assert_eq!(result, expected_struct);
    }

    // #[test]
    // fn peek_token() {
    //     let mut iter = TokenIter::from(vec![
    //         t!(int),
    //         Token::Identifier("variable".to_string()),
    //         t!( = ),
    //         Token::LiteralInt(2),
    //         t!( ; ),
    //     ]);
    //     let rint = iter.peek_token(t!(int));
    //     let rident = iter.peek_token(t!(ident));
    //     assert!(rint.is_ok());
    //     assert!(rident.is_err());
    //     assert_eq!(iter.current, 0);
    //     iter.increment();

    //     let rident = iter.peek_token(t!(ident));
    //     assert!(rident.unwrap() == Token::Identifier("variable".to_string()));
    //     assert_eq!(iter.current, 1);
    // }

    #[test]
    fn test_push_pop() {
        let tokens = vec![
            t!(int),
            Token::Identifier("variable".to_string()),
            t!( = ),
            Token::LiteralInt(2),
            t!( ; ),
        ];
        let mut iter = TokenIter::new(tokens);
        iter.push();
        assert_eq!(iter.stack, vec![0]);
        let r = iter.expect(|tok| matches!(tok,t!(int)));
        assert!(r.is_ok());
        assert_eq!(iter.current, 1);
        iter.pop();
        assert_eq!(iter.current, 0);
    }

    #[test]
    fn test_expect_empty_tokenlist() {
        let tokens = vec![];
        let mut iter = TokenIter::new(tokens);

        let result = iter.expect(|tok|matches!(tok,t!(l_paren)));
        assert!(result.is_err());
        assert!(iter.current == 0);
    }

    #[test]
    fn test_expect() {
        let tokens = vec![t!(l_paren), t!(r_paren), t!(,), t!(litint 4)];
        let mut iter = TokenIter::new(tokens);

        let lparen_r = iter.expect(|tok|matches!(tok,t!(l_paren)));
        assert!(lparen_r.unwrap() == t!(l_paren));

        let rparen_r = iter.expect(|tok|matches!(tok,t!(r_paren)));
        assert!(rparen_r.unwrap() == t!(r_paren));

        let comma_r = iter.expect(|tok|matches!(tok,t!( , )));
        assert_eq!(comma_r.unwrap(), t!( , ));

        let litint_r = iter.expect(|tok|matches!(tok,t!(litint)));
        assert!(litint_r.unwrap() == t!(litint 4));

        assert!(iter.current == 4)
    }

    #[test]
    fn test_new() {
        let tokens = vec![
            t!(l_paren),
            t!(litint 21),
            t!(,),
            t!(litint 2),
            t!(,),
            t!(litint 21),
            t!(,),
            t!(litint 2),
            t!(,),
            t!(r_paren),
        ];
        let iter = TokenIter::new(tokens);
        assert_eq!(iter.current, 0);
        assert_eq!(iter.tokens.len(), 10);
        // assert_eq!(iter.size, 0); // how to test private method?
        assert_eq!(iter.stack.len(), 0);
    }

    #[test]
    fn test_new_empty() {
        let tokens = vec![];
        let iter = TokenIter::<Token>::new(tokens);
        assert_eq!(iter.current, 0);
        assert_eq!(iter.tokens.len(), 0);
        // assert_eq!(iter.size, 0); // how to test private method?
        assert_eq!(iter.stack.len(), 0);
    }
}
