use nom::IResult;

use crate::{
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndOfFile;
impl<'a> Parser<&'a str> for EndOfFile {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        if input.is_empty() {
            Ok((input, Self))
        } else {
            Err(nom::Err::Error(nom::error::Error::new(
                input,
                nom::error::ErrorKind::Eof,
            )))
        }
    }
}

impl<V: Visitor> Visitable<V> for EndOfFile {
    default fn accept(&self, _: &mut V) {}
}
impl<V: VisitorMut> VisitableMut<V> for EndOfFile {
    default fn accept_mut(&mut self, _: &mut V) {}
}
