use nom::{
    combinator::{map, peek},
    sequence::terminated,
    IResult,
};

use crate::{
    ast::{terminal::identifier::Identifier, trivia::with_trivia::WithTrivia},
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeFollowedById(pub Type);
impl<'a> Parser<&'a str> for TypeFollowedById {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            terminated(Type::parse, peek(WithTrivia::<Identifier>::parse)),
            |ty| Self(ty),
        )(input)
    }
}

impl<V: Visitor> Visitable<V> for TypeFollowedById {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.0);
    }
}

impl<V: VisitorMut> VisitableMut<V> for TypeFollowedById {
    default fn accept_mut(&mut self, v: &mut V) {
        self.0.accept_mut(v);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type(pub Identifier);

impl<'a> Parser<&'a str> for Type {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(Identifier::parse, |id| Self(id))(input)
    }
}
impl<V: Visitor> Visitable<V> for Type {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.0)
    }
}
impl<V: VisitorMut> VisitableMut<V> for Type {
    default fn accept_mut(&mut self, v: &mut V) {
        self.0.accept_mut(v);
    }
}
