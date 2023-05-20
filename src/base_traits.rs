use crate::{ParseError, TokenIter};

pub trait ConsumableToken: Clone {
    fn stateless_equals(&self, other: &Self) -> bool;
}

pub trait Parsable<T>
where
    T: ConsumableToken,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized;

    fn identifier() -> String {
        std::any::type_name::<Self>().to_string()
    }
}

pub trait Expectable<T>
where
    T: ConsumableToken,
{
    fn expect<F: Fn(&T) -> bool>(iter: &mut TokenIter<T>, matches: F) -> Result<Self, ParseError<T>>
    where
        Self: Sized;

    fn identifier() -> String {
        std::any::type_name::<Self>().to_string()
    }
}
