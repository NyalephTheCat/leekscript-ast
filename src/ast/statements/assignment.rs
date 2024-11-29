use nom::{combinator::map, sequence::pair, IResult};

use crate::{
    ast::{
        expressions::Expression,
        terminal::{
            identifier::Identifier,
            symbol::{Comma, Equal},
        },
        trivia::with_trivia::WithTrivia,
        utils::separated::Separated1,
    },
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentList(pub Separated1<WithTrivia<Assignment>, WithTrivia<Comma>>);
impl<'a> Parser<&'a str> for AssignmentList {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(<_ as Parser<&'a str>>::parse, Self)(input)
    }
}
impl<V: Visitor> Visitable<V> for AssignmentList {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.0);
    }
}
impl<V: VisitorMut> VisitableMut<V> for AssignmentList {
    default fn accept_mut(&mut self, v: &mut V) {
        self.0.accept_mut(v);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment(
    Identifier,
    Option<(WithTrivia<Equal>, WithTrivia<Expression>)>,
);
impl<'a> Parser<&'a str> for Assignment {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            pair(<_ as Parser<&'a str>>::parse, <_ as Parser<&'a str>>::parse),
            |(first, last)| Self(first, last),
        )(input)
    }
}
impl<V: Visitor> Visitable<V> for Assignment {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.0);
        v.visit(&self.1);
    }
}
impl<V: VisitorMut> VisitableMut<V> for Assignment {
    default fn accept_mut(&mut self, v: &mut V) {
        self.0.accept_mut(v);
        self.1.accept_mut(v);
    }
}
