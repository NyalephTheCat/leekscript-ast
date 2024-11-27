use std::marker::PhantomData;

use nom::{combinator::opt, multi::many0, sequence::tuple, IResult, InputLength};



pub trait Parser<I>: Sized {
    fn parse(input: I) -> IResult<I, Self>;
}

impl<I> Parser<I> for () {
    fn parse(input: I) -> IResult<I, Self> {
        Ok((input, ()))
    }
}

impl<I, T1> Parser<I> for (T1,)
where
    I: Clone,
    T1: Parser<I>,
{
    fn parse(input: I) -> IResult<I, Self> {
        T1::parse(input).map(|(rest, item)| (rest, (item,)))
    }
}

impl<I, T1, T2> Parser<I> for (T1, T2)
where
    I: Clone,
    T1: Parser<I>,
    T2: Parser<I>,
{
    fn parse(input: I) -> IResult<I, Self> {
        tuple((T1::parse, T2::parse))(input)
    }
}

impl<I, T1, T2, T3> Parser<I> for (T1, T2, T3)
where
    I: Clone,
    T1: Parser<I>,
    T2: Parser<I>,
    T3: Parser<I>,
{
    fn parse(input: I) -> IResult<I, Self> {
        tuple((T1::parse, T2::parse, T3::parse))(input)
    }
}

impl<I, T1, T2, T3, T4> Parser<I> for (T1, T2, T3, T4)
where
    I: Clone,
    T1: Parser<I>,
    T2: Parser<I>,
    T3: Parser<I>,
    T4: Parser<I>,
{
    fn parse(input: I) -> IResult<I, Self> {
        tuple((T1::parse, T2::parse, T3::parse, T4::parse))(input)
    }
}

impl<I, T> Parser<I> for PhantomData<T> {
    fn parse(input: I) -> IResult<I, Self> {
        Ok((input, PhantomData))
    }
}

impl<I, T> Parser<I> for Option<T>
where
    I: Clone,
    T: Parser<I>,
{
    fn parse(input: I) -> IResult<I, Self> {
        opt(T::parse)(input)
    }
}

impl<I, T> Parser<I> for Vec<T>
where
    I: Clone + InputLength,
    T: Parser<I>,
{
    fn parse(input: I) -> IResult<I, Self> {
        many0(T::parse)(input)
    }
}

impl<I, T> Parser<I> for Box<T>
where
    T: Parser<I>,
{
    fn parse(input: I) -> IResult<I, Self> {
        T::parse(input).map(|(rest, item)| (rest, Box::new(item)))
    }
}
