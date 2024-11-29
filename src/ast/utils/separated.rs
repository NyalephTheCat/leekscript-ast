use nom::{
    error::{Error, ErrorKind},
    IResult,
};

use crate::visitor::{Visitable, VisitableMut, Visitor, VisitorMut};

use crate::parser::Parser;

pub type Separated0<T, S> = Separated<T, S, 0, { usize::MAX }>;
pub type Separated1<T, S> = Separated<T, S, 1, { usize::MAX }>;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Separated<T, S, const MIN: usize, const MAX: usize>(pub Vec<T>, pub Vec<S>);
impl<I, T, S, const MIN: usize, const MAX: usize> Parser<I> for Separated<T, S, MIN, MAX>
where
    I: Clone,
    T: Parser<I>,
    S: Parser<I>,
{
    fn parse(mut input: I) -> IResult<I, Self> {
        let mut items = Vec::new();
        let mut separators = Vec::new();
        for i in 0..MAX {
            match T::parse(input.clone()) {
                Ok((rest, item)) => {
                    items.push(item);
                    input = rest;
                }
                Err(_) => break, // Stop on first error
            }
            if i == MAX - 1 {
                break;
            }
            match S::parse(input.clone()) {
                Ok((rest, sep)) => {
                    separators.push(sep);
                    input = rest;
                }
                Err(_) => break, // Stop on first error
            }
        }
        if items.len() < MIN {
            Err(nom::Err::Error(Error::new(input, ErrorKind::Many0)))
        } else {
            Ok((input, Separated(items, separators)))
        }
    }
}

impl<V, T, S, const MIN: usize, const MAX: usize> Visitable<V> for Separated<T, S, MIN, MAX>
where
    V: Visitor,
    T: Visitable<V>,
    S: Visitable<V>,
{
    default fn accept(&self, visitor: &mut V) {
        for (item, sep) in self.0.iter().zip(&self.1) {
            visitor.visit(item);
            visitor.visit(sep);
        }
        visitor.visit(self.0.last().unwrap());
    }
}

impl<V, T, S, const MIN: usize, const MAX: usize> VisitableMut<V> for Separated<T, S, MIN, MAX>
where
    V: VisitorMut,
    T: VisitableMut<V>,
    S: VisitableMut<V>,
{
    default fn accept_mut(&mut self, visitor: &mut V) {
        for i in 0..(self.0.len() - 1) {
            visitor.visit_mut(&mut self.0[i]);
            visitor.visit_mut(&mut self.1[i]);
        }
        visitor.visit_mut(self.0.last_mut().unwrap());
    }
}
