use crate::{error::parse_error::ParseError, iter::TokenIter};

pub trait ConsumableToken: PartialEq + Clone{
    fn stateless_equals(&self, other: &Self) -> bool;
}

pub trait Parsable<T>: PartialEq + Clone
where T: ConsumableToken
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where Self:Sized;

    fn identifier() -> String{
        std::any::type_name::<Self>().to_string()
    }
}

pub trait Expectable<T>: PartialEq + Clone
where T: ConsumableToken
{
    fn expect(iter:&mut TokenIter<T>, expected_token: T) -> Result<Self, ParseError<T>>
    where Self:Sized;

    fn identifier() -> String {
        std::any::type_name::<Self>().to_string()
    }
}