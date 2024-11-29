use nom::{combinator::map, IResult};

use crate::{
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::terminal::identifier::Identifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
}
impl<'a> Parser<&'a str> for Expression {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(<_ as Parser<&'a str>>::parse, Self::Identifier)(input)
    }
}
impl<V: Visitor> Visitable<V> for Expression {
    default fn accept(&self, v: &mut V) {
        match self {
            Self::Identifier(node) => v.visit(node),
        }
    }
}
impl<V: VisitorMut> VisitableMut<V> for Expression {
    default fn accept_mut(&mut self, v: &mut V) {
        match self {
            Self::Identifier(node) => node.accept_mut(v),
        }
    }
}
