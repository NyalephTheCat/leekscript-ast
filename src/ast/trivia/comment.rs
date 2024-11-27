use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::{map, opt},
    sequence::tuple,
};

use crate::{
    parser::Parser,
    visitor::{writer::Writer, Visitable, VisitableMut, Visitor, VisitorMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comment {
    SingleLine(String, bool),
    MultiLine(String),
}

impl<'a> Parser<&'a str> for Comment {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        alt((
            map(
                tuple((tag("//"), take_until("\n"), opt(tag("\n")))),
                |(_, content, newline): (_, &'a str, Option<_>)| {
                    Self::SingleLine(content.into(), newline.is_some())
                },
            ),
            map(
                tuple((tag("/*"), take_until("*/"), tag("*/"))),
                |(_, content, _): (_, &'a str, _)| Self::MultiLine(content.into()),
            ),
        ))(input)
    }
}

impl<V: Visitor> Visitable<V> for Comment {
    default fn accept(&self, _: &mut V) {}
}

impl<V: VisitorMut> VisitableMut<V> for Comment {
    default fn accept_mut(&mut self, _: &mut V) {}
}

impl Visitable<Writer> for Comment {
    fn accept(&self, visitor: &mut Writer) {
        match self {
            Self::SingleLine(content, newline) => {
                visitor.0 += &format!("//{}{}", content, if *newline { "\n" } else { "" })
            }
            Self::MultiLine(content) => visitor.0 += &format!("/*{}*/", content),
        }
    }
}
