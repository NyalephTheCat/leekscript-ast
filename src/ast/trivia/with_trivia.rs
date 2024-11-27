use nom::{combinator::map, sequence::tuple};

use crate::{
    ast::utils::repeat::Repeat0,
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::trivia::Trivia;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithTrivia<T>(pub Vec<Trivia>, pub T);

impl<'a, T: Parser<&'a str>> Parser<&'a str> for WithTrivia<T> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        map(
            tuple((Repeat0::<Trivia>::parse, T::parse)),
            |(trivia, item)| Self(trivia.into(), item),
        )(input)
    }
}

impl<V: Visitor, T: Visitable<V>> Visitable<V> for WithTrivia<T> {
    default fn accept(&self, visitor: &mut V) {
        visitor.visit(&self.0);
        visitor.visit(&self.1);
    }
}

impl<V: VisitorMut, T: VisitableMut<V>> VisitableMut<V> for WithTrivia<T> {
    default fn accept_mut(&mut self, visitor: &mut V) {
        self.0.accept_mut(visitor);
        self.1.accept_mut(visitor);
    }
}
