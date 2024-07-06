use std::marker::PhantomData;

use crate::{ParseError, TokenIter};

#[macro_export]
macro_rules! validator {
    ($pattern:pat) => {
        FunctionValidator::new(Matcher(|t|matches!(t, $pattern)))
    };
}

#[macro_export]
macro_rules! matcher {
    ($pattern:pat) => {
        Matcher(|t|matches!(t, $pattern))
    };
}

impl<P> Default for FunctionValidator<P> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

pub struct FunctionValidator<P> {
    matcher: Matcher<P>,
}

impl<P> FunctionValidator<P> {
    pub fn new(matcher: Matcher<P>) -> Self {
        Self { matcher }
    }
}

impl<T: ConsumableToken, P: Parsable<T>> Parser<T, P> for FunctionValidator<P> {
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<P, ParseError> {
        let r = iter.parse()?;
        if (self.matcher)(&r) {
            Ok(r)
        } else {
            Err(ParseError::parsed_but_unmatching(
                iter.current,
                &r,
                // TODO: update error message
                "TODO: check this",
            ))
        }
    }
}

pub trait ConsumableToken: Clone + std::fmt::Debug + Parsable<Self> {}

pub trait Parsable<T>
where
    Self: Sized + std::fmt::Debug,
    T: ConsumableToken,
{
    fn parser() -> impl Parser<T, Self> {
        NoOpParser
    }

}

#[derive(Clone)]
pub struct Matcher<P>(pub fn(&P) -> bool);

impl<P> Default for Matcher<P> {
    fn default() -> Self {
        Self(|_| true)
    }
}

impl<P> std::ops::Deref for Matcher<P> {
    type Target = fn(&P) -> bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait Parser<T, P>: Default
where
    T: ConsumableToken,
    P: Parsable<T>,
{
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<P, ParseError>;
}

#[derive(Default)]
pub struct NoOpParser;

impl<T: ConsumableToken, P: Parsable<T>> Parser<T, P> for NoOpParser {
    fn parse(&self, iter: &mut TokenIter<T>) -> Result<P, ParseError> {
        iter.parse()
    }
}
