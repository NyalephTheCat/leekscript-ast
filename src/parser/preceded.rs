use nom::{combinator::map, sequence::tuple, IResult};

use crate::visitor::{Visitable, VisitableMut, Visitor, VisitorMut};

use super::Parser;

pub struct Preceded<Term, T>(pub Term, pub T);
impl<I, Prec, T> Parser<I> for Preceded<Prec, T>
where
    I: Clone,
    Prec: Parser<I>,
    T: Parser<I>,
{
    fn parse(input: I) -> IResult<I, Self> {
        map(tuple((Prec::parse, T::parse)), |(prec, t)| Self(prec, t))(input)
    }
}

impl<V, Prec, T> Visitable<V> for Preceded<Prec, T>
where
    V: Visitor,
    Prec: Visitable<V>,
    T: Visitable<V>,
{
    default fn accept(&self, visitor: &mut V) {
        visitor.visit(&self.0);
        visitor.visit(&self.1);
    }
}

impl<V, Prec, T> VisitableMut<V> for Preceded<Prec, T>
where
    V: VisitorMut,
    Prec: VisitableMut<V>,
    T: VisitableMut<V>,
{
    default fn accept_mut(&mut self, visitor: &mut V) {
        visitor.visit_mut(&mut self.0);
        visitor.visit_mut(&mut self.1);
    }
}
