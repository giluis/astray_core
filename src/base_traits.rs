use crate::{ParseError, TokenIter};

pub trait ConsumableToken: Clone + std::fmt::Debug + Parsable<Self> {}

pub trait Parsable<T>
where
    Self: Sized + std::fmt::Debug,
    T: ConsumableToken,
{

    type V: Validator<T, Self> = NoOpValidator;
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>;

    fn validator() -> Self::V {
        Self::V::default()
    }

    fn identifier() -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub type Matcher<T> = fn (&T) -> bool;

#[allow(non_snake_case)]
fn DEFAULT_MATCHER<T>(arg: &T) -> bool {
    true
}

pub trait Validator<T, P>: Default
where T: ConsumableToken, P: Parsable<T>
{
    fn validate(&self, iter: &mut TokenIter<T>) -> Result<P, ParseError<T>>;
}

pub struct NoOpValidator;

impl <T: ConsumableToken, P: Parsable<T>> Validator<T, P> for NoOpValidator {
    fn validate(&self, iter: &mut TokenIter<T>) -> Result<P, ParseError<T>>{
        iter.parse()
    }
}
