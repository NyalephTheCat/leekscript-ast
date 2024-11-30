use nom::{branch::alt, combinator::map, IResult};

use crate::{
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::terminal::{
    identifier::Identifier, number_literal::NumberLiteral, string_literal::StringLiteral,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    StringLiteral(StringLiteral),
    NumberLiteral(NumberLiteral),
}
impl<'a> Parser<&'a str> for Expression {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            map(<_ as Parser<&'a str>>::parse, Self::Identifier),
            map(<_ as Parser<&'a str>>::parse, Self::StringLiteral),
            map(<_ as Parser<&'a str>>::parse, Self::NumberLiteral),
        ))(input)
    }
}
impl<V: Visitor> Visitable<V> for Expression {
    default fn accept(&self, v: &mut V) {
        match self {
            Self::Identifier(node) => v.visit(node),
            Self::StringLiteral(node) => v.visit(node),
            Self::NumberLiteral(node) => v.visit(node),
        }
    }
}
impl<V: VisitorMut> VisitableMut<V> for Expression {
    default fn accept_mut(&mut self, v: &mut V) {
        match self {
            Self::Identifier(node) => node.accept_mut(v),
            Self::StringLiteral(node) => node.accept_mut(v),
            Self::NumberLiteral(node) => node.accept_mut(v),
        }
    }
}
