use crate::base_traits::{Matcher, Parsable, Parser};
use crate::error::ParseError;
use crate::iter::TokenIter;
use crate::matcher;
use crate::t;
use crate::token::Token;
use derive_builder::Builder;

#[derive(PartialEq, Debug, Clone)]
pub struct TestStruct {
    pub var_type: Token,
    pub var_name: String,
    pub equals_sign: Token,
    pub value: u32,
}

impl Parsable<Token> for TestStruct {
    fn parser() -> impl Parser<Token, Self> {
        TestStructParser::default()
    }
}

#[derive(Default, Builder)]
pub struct TestStructParser {}

impl Parser<Token, TestStruct> for TestStructParser {
    fn parse<'a>(&self, iter: &mut TokenIter<Token>) -> Result<TestStruct, ParseError> {
        iter.try_do(|token_iter| {
            let var_type = Token::parser()
                .with_matcher(matcher!(t!(int)))
                .parse(token_iter)?;

            let var_name = match Token::parser()
                .with_matcher(matcher!(Token::Identifier(_)))
                .parse(token_iter)?
            {
                Token::Identifier(ident_str) => ident_str,
                _ => unreachable!("Internal error, should be ident_str"),
            };

            let equals_sign = Token::parser()
                .with_matcher(matcher!(t!(=)))
                .parse(token_iter)?;

            let value = match Token::parser()
                .with_matcher(matcher!(Token::LiteralInt(_)))
                .parse(token_iter)?
            {
                Token::LiteralInt(ident_str) => ident_str,
                _ => unreachable!("Internal error, should be ltoken_iteral int"),
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
