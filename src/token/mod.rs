use std::cmp::max;

use crate::{ConsumableToken, Parsable, ParseError, TokenIter};

#[derive(PartialEq, Default, Debug, Clone)]
pub struct LiteralStringValue {
    value: String,
}

impl From<String> for LiteralStringValue {
    fn from(s: String) -> Self {
        LiteralStringValue { value: s }
    }
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct LiteralIntValue {
    value: String,
}

impl From<String> for LiteralIntValue {
    fn from(s: String) -> Self {
        LiteralIntValue { value: s }
    }
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct IdentifierValue {
    value: String,
}

impl From<String> for IdentifierValue {
    fn from(s: String) -> Self {
        IdentifierValue { value: s }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Operations
    Assign,
    Plus,
    Minus,
    Mult,
    Div,
    // KeyWords and Literals
    KInt,
    KFloat,
    KReturn,
    // Literals
    LiteralString(String),
    LiteralInt(u32),
    //Identifier
    Identifier(String),
    // Delimeters
    RCurly,
    LCurly,
    RBracket,
    LBracket,
    RParen,
    LParen,
    Comma,
    //Punctuation
    SemiColon,
    // meta
    INVALID,
    EMPTY,
}

impl Token {
    pub fn from_regex_result(&self, input: String) -> (Token, usize) {
        let token = match *self {
            Token::LiteralString(_) => Token::LiteralString(input[1..input.len() - 1].to_string()), // remove  quotes around string
            Token::Identifier(_) => Token::Identifier(input.clone()), // remove  quotes around string
            Token::LiteralInt(_) => {
                let s: u32 = input.parse().unwrap();
                Token::LiteralInt(s)
            }
            _ => (*self).clone(),
        };
        (token, input.len())
    }
}

#[macro_export]
macro_rules! t {
    (,) => {
        Token::Comma
    };
    (, def) => {
        ","
    };
    (;) => {
        Token::SemiColon
    };
    (; def) => {
        ";"
    };
    (=) => {
        Token::Assign
    };
    (= def) => {
        "="
    };
    (+) => {
        Token::Plus
    };
    (+ def) => {
        "+"
    };
    (-) => {
        Token::Minus
    };
    (- def) => {
        "-"
    };
    (*) => {
        Token::Mult
    };
    (* def) => {
        "*"
    };
    (/) => {
        Token::Div
    };
    (/ def) => {
        "/"
    };
    (int) => {
        Token::KInt
    };
    (float) => {
        Token::KFloat
    };
    (int def) => {
        "int"
    };
    (return) => {
        Token::KReturn
    };
    (return def) => {
        "return"
    };

    (litstr) => {
        Token::LiteralString("DEFAULT_LITERAL_STRING")
    };
    (litint) => {
        Token::LiteralInt(0)
    };
    (ident) => {
        Token::Identifier("DEFAULT_IDENTIFIER".to_string())
    };

    (litstr $value:expr) => {
        Token::LiteralString($value.to_string())
    };
    (litint $value:expr) => {
        Token::LiteralInt($value)
    };
    (ident $value:expr) => {
        Token::Identifier($value.to_string())
    };

    (r_paren) => {
        Token::RParen
    };
    (r_paren def) => {
        ")"
    };
    (l_paren) => {
        Token::LParen
    };
    (l_paren def) => {
        "("
    };
    (r_curly) => {
        Token::RCurly
    };
    (r_curly def) => {
        "}"
    };
    (l_curly) => {
        Token::LCurly
    };
    (l_curly def) => {
        "{"
    };
    (r_bracket) => {
        Token::RBracket
    };
    (r_bracket def) => {
        "]"
    };
    (l_bracket) => {
        Token::LBracket
    };
    (l_bracket def) => {
        "["
    };
    (empty) => {
        Token::EMPTY
    };
    (invalid) => {
        Token::INVALID
    };
}

impl ConsumableToken for Token {}

impl Parsable<Token> for Token {
    fn parse(iter: &mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
    where
        Self: Sized,
    {
        match iter.consume() {
            Some(token) => Ok(token),
            None => Err(ParseError::no_more_tokens::<Token>(iter.current)),
        }
    }

    fn parse_if_match<F: Fn(&Token) -> bool>(
        iter: &mut TokenIter<Token>,
        matches: F,
        pattern: Option<&'static str>
    ) -> Result<Self, ParseError<Token>>
    where
        Self: Sized,
    {
        // This TODO seems to be outdated and deprecated
        // Comparison will always be necessary. The question is whether or not 
        // matches! is the best way to achieve this use case ,which I think it is.
        // Patterns cannot be passed as values, so a match function is needed
        // TODO: find a way to express this that doesn't need matching:
        // this introduces overhead every time a token is parsed
        iter.try_do(|token_iter| match token_iter.consume() {
            Some(ref found) if matches(found) => Ok(found.clone()),
            Some(ref found) => Err(ParseError::parsed_but_unmatching::<Token>(
                if token_iter.current == 0 {
                    token_iter.current
                } else {
                    token_iter.current - 1
                },
                found,
                pattern
            )),
            _ => Err(ParseError::no_more_tokens::<Token>(if token_iter.current == 0 {
                token_iter.current
            } else {
                token_iter.current - 1
            })),
        })
    }
}
