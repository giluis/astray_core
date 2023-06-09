use std::mem::MaybeUninit;

use arr_macro::arr;

use crate::{
    base_traits::Parsable, error::parse_error::ParseError, iter::TokenIter, ConsumableToken,
};

impl<P1, P2, T> Parsable<T> for (P1, P2)
where
    P1: Parsable<T>,
    P2: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        Ok((iter.parse()?, iter.parse()?))
    }
}

impl<P1, P2, P3, T> Parsable<T> for (P1, P2, P3)
where
    P1: Parsable<T>,
    P2: Parsable<T>,
    P3: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        Ok((
            iter.parse::<P1>()?,
            iter.parse::<P2>()?,
            iter.parse::<P3>()?,
        ))
    }
}

// #[macro_export]
// macro_rules! impl_tuple {
//     ($($tuplll:ty),*) => {
//         impl<$($tuplll,)* T> Parsable<T> for ($($tuplll),*)
//         where
//             $($tuplll: Parsable<T>,)*
//             T: ConsumableToken,
//         {
//             fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
//             where
//                 Self: Sized,
//             {
//                 Ok(($(iter.parse::<$tuplll>()?,)*))
//             }
//         }
//     };
//     () => ()
// }

impl<T, P> Parsable<T, P> for Box<P>
where
    P: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        Ok(Box::new(P::parse(iter)?))
    }

    fn parse_if_match<F: Fn(&P) -> bool>(
        iter: &mut TokenIter<T>,
        matches: F,
    ) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        Ok(Box::new(P::parse_if_match(iter, matches)?))
    }
}

impl<T, P> Parsable<T, P> for Vec<P>
where
    P: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>> {
        let mut results = vec![];
        while let Ok(r) = P::parse(iter) {
            results.push(r);
        }
        Ok(results)
    }

    fn parse_if_match<F>(iter: &mut TokenIter<T>, matches: F) -> Result<Vec<P>, ParseError<T>>
    where
        F: Fn(&P) -> bool,
    {
        Ok(iter.parse_while(matches))
    }
}

impl<T, P> Parsable<T, P> for Option<P>
where
    P: Parsable<T>,
    T: ConsumableToken,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>> {
        let r = P::parse(iter);
        match r {
            Ok(r) => Ok(Some(r)),
            Err(_) => Ok(None),
        }
    }

    fn parse_if_match<F: Fn(&P) -> bool>(
        iter: &mut TokenIter<T>,
        matches: F,
    ) -> Result<Self, ParseError<T>>
    where
        Self: Sized,
    {
        Self::parse(iter).map(|result| {
            if let Some(ref inner) = result && matches(inner) {
                result
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        base_traits::Parsable, error::parse_error::ParseError, iter::TokenIter, t, token::Token,
    };

    #[derive(Debug, PartialEq, Clone)]
    struct TestStruct {
        ident: String,
        semi: Option<Token>,
    }

    impl Parsable<Token> for TestStruct {
        fn parse(iter: &mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
        where
            Self: Sized,
        {
            let ident= match iter.parse_if_match(|tok|matches!(tok, Token::Identifier(_)))?{
                Token::Identifier(string) => string,
                _ => unreachable!("Domain error: token returned by parse_if_match should be of the same variant as the token passed as argument"),
            };
            let semi = <Option<Token> as Parsable<Token, Token>>::parse_if_match(iter, |tok| {
                matches!(tok, Token::SemiColon)
            })?;
            Ok(TestStruct { ident, semi })
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    struct VecStruct {
        idents: Vec<Token>,
    }

    impl Parsable<Token, Token> for VecStruct {
        fn parse(iter: &mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
        where
            Self: Sized,
        {
            let idents = iter.parse_if_match(|tok| matches!(tok, Token::Identifier(_)))?;
            Ok(VecStruct { idents })
        }
        fn parse_if_match<F: Fn(&Token) -> bool>(
            iter: &mut TokenIter<Token>,
            f: F,
        ) -> Result<Self, ParseError<Token>>
        where
            Self: Sized,
        {
            let idents = iter.parse_if_match(f)?;
            Ok(VecStruct { idents })
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    struct BoxStruct {
        ident: Box<Token>,
    }

    impl Parsable<Token, Token> for BoxStruct {
        fn parse(iter: &mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
        where
            Self: Sized,
        {
            let ident = Box::parse(iter)?;
            Ok(BoxStruct { ident })
        }

        fn parse_if_match<F: Fn(&Token) -> bool>(
            iter: &mut TokenIter<Token>,
            f: F,
        ) -> Result<Self, ParseError<Token>>
        where
            Self: Sized,
        {
            let ident = Box::parse_if_match(iter, f)?;
            Ok(BoxStruct { ident })
        }
    }

    #[test]
    fn box_ok() {
        struct Test1 {
            ident: Token,
        }

        let tokens = vec![t!(ident "hello")];
        let result = BoxStruct::parse(&mut TokenIter::new(tokens)).unwrap();
        assert_eq!(
            result,
            BoxStruct {
                ident: Box::new(t!(ident "hello"))
            }
        );
    }

    #[test]
    fn box_err() {
        let tokens = vec![];
        let result = BoxStruct::parse(&mut TokenIter::new(tokens));
        assert!(result.is_err());
    }

    #[test]
    fn box_match() {
        let tokens = vec![t!(ident "hello")];
        let result = BoxStruct::parse_if_match(&mut TokenIter::new(tokens), |t| {
            matches!(t, Token::Identifier(_))
        });
        assert!(result.is_ok());
    }

    #[test]
    fn vec_parse_if_match() {
        let tokens = vec![t!(ident "ident1"), t!(ident "ident2")];

        let result = VecStruct::parse_if_match(&mut TokenIter::new(tokens), |tok| {
            matches!(tok, Token::Identifier(_))
        })
        .expect("Should be ok");
        assert_eq!(
            result.idents,
            vec![
                Token::Identifier("ident1".to_owned()),
                Token::Identifier("ident2".to_owned())
            ]
        );
    }

    #[test]
    fn parse_vec_with_many_elements() {
        let tokens = vec![t!(ident "ident1"), t!(ident "ident2")];

        let result = VecStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
        assert_eq!(
            result.idents,
            vec![
                Token::Identifier("ident1".to_owned()),
                Token::Identifier("ident2".to_owned())
            ]
        );
    }

    #[test]
    fn parse_vec_with_one_element() {
        let tokens = vec![t!(ident "ident1")];

        let result = VecStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
        assert_eq!(result.idents, vec![Token::Identifier("ident1".to_owned())]);
    }

    #[test]
    fn parse_vec_with_zero_elements() {
        let tokens = vec![];

        let result = VecStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
        assert_eq!(result.idents, vec![]);
    }

    #[test]
    fn parse_option_none() {
        let tokens = vec![t!(ident "ident1")];

        let result = TestStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
        assert_eq!(result.ident, "ident1");
        assert!(result.semi.is_none());
    }

    #[test]
    fn parse_option_some() {
        let tokens = vec![t!(ident "ident1"), t!(;)];

        let result = TestStruct::parse(&mut TokenIter::new(tokens)).expect("Should be ok");
        assert!(result.ident == "ident1");
        assert!(result.semi == Some(Token::SemiColon));
    }
}
