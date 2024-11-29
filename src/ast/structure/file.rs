use nom::{combinator::map, sequence::pair, IResult};

use crate::{
    ast::{
        statements::{GlobalFlag, Statements},
        trivia::with_trivia::WithTrivia,
    },
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::eof::EndOfFile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub statements: Vec<WithTrivia<Statements<GlobalFlag>>>,
    pub eof: WithTrivia<EndOfFile>,
}
impl<'a> Parser<&'a str> for File {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            pair(<_ as Parser<&'a str>>::parse, <_ as Parser<&'a str>>::parse),
            |(statements, eof)| Self { statements, eof },
        )(input)
    }
}
impl<V: Visitor> Visitable<V> for File {
    default fn accept(&self, visitor: &mut V) {
        visitor.visit(&self.statements);
        visitor.visit(&self.eof);
    }
}

impl<V: VisitorMut> VisitableMut<V> for File {
    default fn accept_mut(&mut self, visitor: &mut V) {
        self.statements.accept_mut(visitor);
        self.eof.accept_mut(visitor);
    }
}
