use std::marker::PhantomData;

use nom::{
    error::{Error, ErrorKind},
    IResult,
};

use crate::visitor::{Visitable, VisitableMut, Visitor, VisitorMut};

use crate::parser::Parser;

// Type-level list definitions
#[derive(Debug)]
pub struct Nil;

#[derive(Debug)]
pub struct Cons<Head, Tail>(PhantomData<Head>, PhantomData<Tail>);

pub trait Flag {
    const FLAG: bool;
}
impl Flag for Nil {
    const FLAG: bool = false;
}

// Define the WithFlag trait
pub trait WithFlag<T>
where
    T: Flag,
{
    const HAS: bool;
}

// Base case: Nil has no flags
impl<T: Flag> WithFlag<T> for Nil {
    const HAS: bool = false;
}

// If the head is not the flag, delegate to the tail
impl<Head, Tail, T: Flag> WithFlag<T> for Cons<Head, Tail>
where
    Tail: WithFlag<T>,         // Delegate to
    Head: IsSame<T> + Default, // Ensure U is not T
{
    const HAS: bool = Head::SAME || Tail::HAS;
}

trait IsSame<T> {
    const SAME: bool = false;
}

impl<T> IsSame<T> for T {
    const SAME: bool = true;
}

#[derive(Debug)]
pub struct HasFlag<T, S>(PhantomData<(T, S)>)
where
    S: WithFlag<T>,
    T: Flag;

impl<T, S> Default for HasFlag<T, S>
where
    S: WithFlag<T>,
    T: Flag,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<I, S, T> Parser<I> for HasFlag<T, S>
where
    S: WithFlag<T>,
    T: Flag,
{
    fn parse(input: I) -> IResult<I, Self> {
        // If the flag is present, return a successfull parse,
        // Othersise, return an error
        if <S as WithFlag<T>>::HAS && T::FLAG {
            Ok((input, Self::default()))
        } else {
            Err(nom::Err::Error(Error::new(input, ErrorKind::Alt)))
        }
    }
}

#[derive(Debug)]
pub struct HasNotFlag<T, S>(PhantomData<(T, S)>)
where
    S: WithFlag<T>,
    T: Flag;

impl<T, S> Default for HasNotFlag<T, S>
where
    S: WithFlag<T>,
    T: Flag,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<I, S, T> Parser<I> for HasNotFlag<T, S>
where
    S: WithFlag<T>,
    T: Flag,
{
    fn parse(input: I) -> IResult<I, Self> {
        // If the flag is present, return a successfull parse,
        // Othersise, return an error
        if !<S as WithFlag<T>>::HAS || !T::FLAG {
            Ok((input, Self::default()))
        } else {
            Err(nom::Err::Error(Error::new(input, ErrorKind::Alt)))
        }
    }
}

impl<V, F, S> Visitable<V> for HasFlag<F, S>
where
    V: Visitor,
    F: Flag,
    S: WithFlag<F>,
{
    default fn accept(&self, _: &mut V) {}
}

impl<V, F, S> VisitableMut<V> for HasFlag<F, S>
where
    V: VisitorMut,
    F: Flag,
    S: WithFlag<F>,
{
    default fn accept_mut(&mut self, _: &mut V) {}
}

impl<V, F, S> Visitable<V> for HasNotFlag<F, S>
where
    V: Visitor,
    F: Flag,
    S: WithFlag<F>,
{
    default fn accept(&self, _: &mut V) {}
}

impl<V, F, S> VisitableMut<V> for HasNotFlag<F, S>
where
    V: VisitorMut,
    F: Flag,
    S: WithFlag<F>,
{
    default fn accept_mut(&mut self, _: &mut V) {}
}
