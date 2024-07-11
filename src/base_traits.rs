use std::marker::PhantomData;

use crate::{ParseError, TokenIter};


#[macro_export]
macro_rules! matcher {
    ($pattern:pat) => {
        Pattern{
            fun:|t| {matches!(t, $pattern)},
            pat: stringify!($pattern)
        }
    };
}


#[macro_export]
macro_rules! matcher_ref {
    ($pattern:pat) => {
        Matcher(|t| matches!(&t, $pattern))
    };
}

pub trait ConsumableToken: Clone + std::fmt::Debug + Parsable<Self> {}

pub trait Parsable<T>
where
    Self: Sized + std::fmt::Debug,
    T: ConsumableToken,
{

    type P: Parser<T, Self> = NoOpParser;
    fn parser() -> Self::P{
        Self::P::default()
    }
}

pub struct Pattern<P>{
    pub fun: fn(&P) -> bool,
    pub pat: &'static str
}

impl <P> Clone for Pattern<P> {
    fn clone(&self) -> Self {
        Self{
            fun:self.fun.clone(), 
            pat: self.pat}
    }
}

impl<P> Default for Pattern<P> {
    fn default() -> Self {
        Self{fun:|_| true,pat: "_"}
    }
}

impl<P> std::ops::Deref for Pattern<P> {
    type Target = fn(&P) -> bool;

    fn deref(&self) -> &Self::Target {
        &self.fun
    }
}

impl <P> std::fmt::Debug for Pattern<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pattern").field("pat", &self.pat).finish()
    }
}

pub trait Parser<T, P>: Default
where
    T: ConsumableToken,
    P: Parsable<T>,
{
    // //TODO: change to Target
    // type ParseTarget: Parsable<T> = P;

    fn parse(&self, iter: &mut TokenIter<T>) -> Result<P, ParseError>;
}

#[derive(Default)]
pub struct NoOpParser;

impl<T: ConsumableToken, P: Parsable<T>> Parser<T, P> for NoOpParser {
    fn parse(
        &self,
        iter: &mut TokenIter<T>,
    ) -> Result<P, ParseError> {
        iter.parse::<P>()
    }
}
