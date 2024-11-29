use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{map, not},
    multi::many0,
    sequence::tuple,
};

use crate::{
    parser::Parser,
    visitor::{writer::Writer, Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::keyword::Keywords;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);

impl<'a> Parser<&'a str> for Identifier {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        // Recognise !Keywords ~ [a-zA-Z_][a-zA-Z0-9_]*
        map(
            tuple((
                not(Keywords::parse),
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_")))),
            )),
            |(_, head, tail)| {
                let mut ident = String::from(head);
                ident.push_str(&tail.join(""));
                Self(ident)
            },
        )(input)
    }
}

impl<V: Visitor> Visitable<V> for Identifier {
    default fn accept(&self, _: &mut V) {}
}

impl<V: VisitorMut> VisitableMut<V> for Identifier {
    default fn accept_mut(&mut self, _: &mut V) {}
}

impl Visitable<Writer> for Identifier {
    fn accept(&self, visitor: &mut Writer) {
        visitor.0 += self.0.as_str();
    }
}
