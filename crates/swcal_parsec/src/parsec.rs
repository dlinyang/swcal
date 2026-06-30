/// simple parser combinator module
use std::result::Result;

///  ParseError trace error with string
#[derive(Debug, Default)]
pub struct ParseError {
    pub errors: Vec<String>,
}

impl ParseError {
    #[inline]
    pub fn new(error: String) -> Self {
        Self { errors: vec![error] }
    }
}

pub type ParseResult<I, T, E = ParseError> = Result<(T, I), E>;

// #[inline]
// pub fn parse_result_ok<I, T, E>(i: I, t: T) -> ParseResult<I, T, E> {
//     Ok((t, i))
// }

/// parser combinator type
pub trait ParsecT<I, T, E = ParseError> {
    fn parse(&self, input: I) -> ParseResult<I, T, E>;

    #[inline]
    fn or(&self, g: impl ParsecT<I, T, E>) -> impl ParsecT<I, T, E> where I: Copy{
        move |input| self.parse(input).or_else(|_| g.parse(input))
    }

    fn then<R>(&self, other: impl ParsecT<I, R, E>) -> impl ParsecT<I, R, E> {
        move |input| {
            let (_, rest1) = self.parse(input)?;
            let (v2, rest2) = other.parse(rest1)?;
            Ok((v2, rest2))
        }
    }

    fn terminated<R>(&self, other: impl ParsecT<I, R, E>) -> impl ParsecT<I, T, E> {
        move |input| {
            let (v1, rest1) = self.parse(input)?;
            let (_, rest2) = other.parse(rest1)?;
            Ok((v1, rest2))
        }
    }

    fn preceded<L>(&self, other: impl ParsecT<I, L, E>) -> impl ParsecT<I, T, E> {
        move |input| {
            let (_, rest1) = other.parse(input)?;
            let (v2, rest2) = self.parse(rest1)?;
            Ok((v2, rest2))
        }
    }
}

/// impl parser combinator function
impl<I, T, E, F> ParsecT<I, T, E> for F
where
    F: Fn(I) -> ParseResult<I, T, E>,
{
    #[inline]
    fn parse(&self, input: I) -> ParseResult<I, T, E> {
        (self)(input)
    }
}

#[inline]
pub fn lexeme<I: Copy, T, E>(
    ws: impl ParsecT<I, (), E> + Copy,
    t: impl ParsecT<I, T, E>,
) -> impl ParsecT<I, T, E> {
    move |input| {
        let (_, rest) = many0(ws).parse(input)?;
        let (ret, rest) = t.parse(rest)?;
        Ok((ret, rest))
    }
}

pub fn between<I, T, E> (
    left: impl ParsecT<I, T, E>,
    right: impl ParsecT<I, T, E>,
    inner: impl ParsecT<I, T, E>,
) -> impl ParsecT<I, T, E> {
    move |input| {
        let (_, left_rest) = left.parse(input)?;
        let (inner, inner_rest) = inner.parse(left_rest)?;
        let (_, right_rest) = right.parse(inner_rest)?;
        Ok((inner, right_rest))
    }
}

#[macro_export]
macro_rules! choice {
    ($p1:expr, $p2:expr) => { $p1.or($p2) };
    ($p1:expr, $p2:expr, $($rest:expr),+) => { $p1.or(choice!($p2, $($rest),+)) };
}

#[inline]
pub fn many0<I: Copy, T, E>(f: impl ParsecT<I, T, E>) -> impl ParsecT<I, Vec<T>, E> {
    move |mut input| {
        let mut result = Vec::new();
        while let Ok((val, rest)) = f.parse(input) {
            result.push(val);
            input = rest;
        }
        Ok((result, input))
    }
}

#[inline]
pub fn many<I: Copy, T, E>(f: impl ParsecT<I, T, E>) -> impl ParsecT<I, Vec<T>, E> {
    move |input| match f.parse(input) {
        Ok((val, mut rest)) => {
            let mut result = Vec::new();
            result.push(val);
            while let Ok((val1, rest1)) = f.parse(rest) {
                result.push(val1);
                rest = rest1;
            }
            Ok((result, rest))
        }
        Err(e) => Err(e),
    }
}

// pub fn try_<'a, T>(f: impl ParsecT<'a, T>, input: &'a str) -> ParseResult<'a, T> {
//     match f.parse(input) {
//         Ok((val, rest)) => Ok((val, rest)),
//         Err(err) => Err(err),
//     }
// }
