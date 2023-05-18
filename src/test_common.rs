use crate::{Token, Parsable, TokenIter, ParseError, t};

#[derive(PartialEq, Debug, Clone)]
pub struct TestStruct {
    pub var_type: Token,
    pub var_name: String,
    pub equals_sign: Token,
    pub value: u32,
}

impl Parsable<Token> for TestStruct {
    fn parse<'a>(iter: &mut TokenIter<Token>) -> Result<TestStruct, ParseError<Token>> {
        iter.try_do(|token_iter| {
            let var_type = token_iter.expect(|token|matches!(token,t!(int)))?;

            let var_name = match token_iter.expect(|token|matches!(token, Token::Identifier(_)))? {
                Token::Identifier(ident_str) => ident_str,
                _ => panic!("Internal error, should be ident_str"),
            };

            let equals_sign = token_iter.expect(|token|matches!(token,t!( = ))).unwrap();
            let value = match token_iter.expect(|token|matches!(token,t!(litint))).unwrap() {
                Token::LiteralInt(value) => value,
                _ => panic!("Internal error: should be lit int"),
            };
            Ok(TestStruct {
                var_type,
                var_name,
                equals_sign,
                value,
            })
        })
    }
}