use crate::{error::parse_error::ParseError, iter::TokenIter};
pub trait Parsable<T>: PartialEq 
{
    fn expect(iter: &mut TokenIter<T>, p: Self) -> Result<Self, ParseError<Self,T>>
    where
        Self: Sized {
        // <Self as Parsable<T>>::parse(iter)
        // .and_then(|r| r.default())
        todo!()

    }

    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<Self,T>>
    where Self:Sized;

    fn identifier() -> syn::Ident;

}


