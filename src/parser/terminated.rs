use nom::{combinator::map, sequence::tuple, IResult};

use crate::visitor::{Visitable, VisitableMut, Visitor, VisitorMut};

use super::Parser;

pub struct Terminated<T, Term>(pub T, pub Term);
impl<I, T, Term> Parser<I> for Terminated<T, Term>
where
    I: Clone,
    T: Parser<I>,
    Term: Parser<I>,
{
    fn parse(input: I) -> IResult<I, Self> {
        map(tuple((T::parse, Term::parse)), |(item, term)| {
            Self(item, term)
        })(input)
    }
}

impl<V, T, Term> Visitable<V> for Terminated<T, Term>
where
    V: Visitor,
    T: Visitable<V>,
    Term: Visitable<V>,
{
    default fn accept(&self, visitor: &mut V) {
        visitor.visit(&self.0);
        visitor.visit(&self.1);
    }
}

impl<V, T, Term> VisitableMut<V> for Terminated<T, Term>
where
    V: VisitorMut,
    T: VisitableMut<V>,
    Term: VisitableMut<V>,
{
    default fn accept_mut(&mut self, visitor: &mut V) {
        visitor.visit_mut(&mut self.0);
        visitor.visit_mut(&mut self.1);
    }
}
