use nom::{
    error::{Error, ErrorKind},
    IResult,
};

use crate::visitor::{Visitable, VisitableMut, Visitor, VisitorMut};

use crate::parser::Parser;

pub type Repeat0<T> = Repeat<T, 0, { usize::MAX }>;
pub type Repeat1<T> = Repeat<T, 1, { usize::MAX }>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Repeat<T, const MIN: usize, const MAX: usize>(pub Vec<T>);
impl<T, const MIN: usize, const MAX: usize> From<Repeat<T, MIN, MAX>> for Vec<T> {
    fn from(val: Repeat<T, MIN, MAX>) -> Self {
        val.0
    }
}

impl<I, T, const MIN: usize, const MAX: usize> Parser<I> for Repeat<T, MIN, MAX>
where
    I: Clone,
    T: Parser<I>,
{
    fn parse(mut input: I) -> IResult<I, Self> {
        let mut items = Vec::new();
        for _ in 0..MAX {
            match T::parse(input.clone()) {
                Ok((rest, item)) => {
                    items.push(item);
                    input = rest;
                }
                Err(_) => break, // Stop on first error
            }
        }
        if items.len() < MIN {
            Err(nom::Err::Error(Error::new(input, ErrorKind::Many0)))
        } else {
            Ok((input, Repeat(items)))
        }
    }
}

impl<V, T, const MIN: usize, const MAX: usize> Visitable<V> for Repeat<T, MIN, MAX>
where
    V: Visitor,
    T: Visitable<V>,
{
    default fn accept(&self, visitor: &mut V) {
        for item in &self.0 {
            visitor.visit(item);
        }
    }
}

impl<V, T, const MIN: usize, const MAX: usize> VisitableMut<V> for Repeat<T, MIN, MAX>
where
    V: VisitorMut,
    T: VisitableMut<V>,
{
    default fn accept_mut(&mut self, visitor: &mut V) {
        for item in &mut self.0 {
            visitor.visit_mut(item);
        }
    }
}
