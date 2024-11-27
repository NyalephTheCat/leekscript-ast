use nom::combinator::map;

use crate::{
    parser::Parser,
    visitor::{writer::Writer, Visitable, VisitableMut, Visitor, VisitorMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Whitespace(pub String);

impl<'a> Parser<&'a str> for Whitespace {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        map(nom::character::complete::multispace1, |ws: &'a str| {
            Self(ws.into())
        })(input)
    }
}

impl<V: Visitor> Visitable<V> for Whitespace {
    default fn accept(&self, _: &mut V) {}
}

impl<V: VisitorMut> VisitableMut<V> for Whitespace {
    default fn accept_mut(&mut self, _: &mut V) {}
}

impl Visitable<Writer> for Whitespace {
    fn accept(&self, visitor: &mut Writer) {
        visitor.0 += &self.0;
    }
}
