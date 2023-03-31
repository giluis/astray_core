use crate::{
    iter::TokenIter,
    base_traits::Parsable, error::parse_error::ParseError,
};
impl<T, P> Parsable<T> for Vec<P>
where
    P: Parsable<T>,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    {
        let mut results = vec![];
        while let Ok(r) = P::parse(iter) {
            results.push(r);
        }
        Ok(results)
    }
}

impl<T, P> Parsable<T> for Option<P>
where
    P: Parsable<T>,
{
    fn parse(iter: &mut TokenIter<T>) -> Result<Self, ParseError<T>>
    {
        let r = P::parse(iter);
        match r {
            Ok(r) => Ok(Some(r)),
            Err(_) => unimplemented!("Error types are necessary here"),
        }
    }
}




#[cfg(test)]
mod tests {

    use crate::{t,base_traits::Parsable, iter::TokenIter, error::parse_error::ParseError, token::Token};


    #[derive(Debug,PartialEq)]
    struct TestStruct {
        ident: String,
        semi: Option<Token>
    }

    impl Parsable<Token> for TestStruct 
    {
        fn parse(iter: & mut TokenIter<Token>) -> Result<Self, ParseError<Token>>
    where
        Self: Sized {
            let ident = match Token::expect(iter,Token::Identifier(Default::default())).map_err(<ParseError<Token>>::from_leaf)? {
                Token::Identifier(string) => string,
                _ => unreachable!("Domain error: token returned by expect should be of the same variant as the token passed as argument"),
            };
            let semi = <Option<Token> as Parsable<Token>>::expect(iter, Some(Token::SemiColon)).map_err(<ParseError<Token>>::from_branch)?;
            Ok(TestStruct {
                ident,
                semi
            })
        }
    }

    #[test]
    fn parse_option_none(){
        let tokens = [
            t!(ident "ident1")
        ];

        let result = TestStruct::parse(&mut TokenIter::new(&tokens)).expect("Should be ok");
        assert!(result.ident == "ident1");
        assert!(result.semi.is_none());
    }

    #[test]
    fn parse_option_some(){
        let tokens = [
            t!(ident "ident1"),
            t!(;),
        ];

        let result = TestStruct::parse(&mut TokenIter::new(&tokens)).expect("Should be ok");
        assert!(result.ident == "ident1");
        assert!(result.semi == Some(Token::SemiColon));
    }

}