use crate::{ParseError, TokenIter};

pub trait ConsumableToken: Clone {
    fn stateless_equals(&self, other: &Self) -> bool;
}

impl<T> Parsable<T> for T
where
    T: ConsumableToken,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        match iter.consume() {
            Some(token) => Ok(token),
            None => Err(ParseError::no_more_tokens(iter.current))
        }
    }

    fn parse_if_match<F: Fn(&T) -> bool>(
        iter: &mut TokenIter<T>,
        matches: F,
    ) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        match iter.consume() {
            Some(ref found) if matches(found) => Ok(found.clone()),
            Some(ref found) => Err(ParseError::unmatching_token(
                iter.current,
                "Failed to expected token not found".to_string(),
                found.clone(),
            )),
            _ => Err(ParseError::no_more_tokens(iter.current)),
        }
    }
}

pub trait Parsable<TToken, P = Self>
where
    TToken: ConsumableToken + Parsable<TToken>,
    P: Parsable<TToken>,
{
    fn parse(iter: &mut TokenIter<TToken>) -> Result<Self, ParseError<TToken>>
    where
        Self: Sized;

    fn parse_if_match<F: Fn(&P) -> bool>(
        iter: &mut TokenIter<TToken>,
        _matches: F,
    ) -> Result<Self, ParseError<TToken>>
    where
        Self: Sized,
    {
        Self::parse(iter)
    }

    fn identifier() -> String {
        std::any::type_name::<Self>().to_string()
    }
}



pub trait Expectable<T>
where
    T: ConsumableToken,
{
    fn identifier() -> String {
        std::any::type_name::<Self>().to_string()
    }
}
