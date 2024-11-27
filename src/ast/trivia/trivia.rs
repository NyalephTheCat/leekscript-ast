use nom::{branch::alt, combinator::map};

use crate::{
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::{comment::Comment, whitespace::Whitespace};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trivia {
    Whitespace(Whitespace),
    Comment(Comment),
}

impl<'a> Parser<&'a str> for Trivia {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        alt((
            map(Whitespace::parse, Self::Whitespace),
            map(Comment::parse, Self::Comment),
        ))(input)
    }
}

impl<V: Visitor> Visitable<V> for Trivia {
    default fn accept(&self, visitor: &mut V) {
        match self {
            Self::Whitespace(whitespace) => visitor.visit(whitespace),
            Self::Comment(comment) => visitor.visit(comment),
        }
    }
}

impl<V: VisitorMut> VisitableMut<V> for Trivia {
    default fn accept_mut(&mut self, visitor: &mut V) {
        match self {
            Self::Whitespace(whitespace) => whitespace.accept_mut(visitor),
            Self::Comment(comment) => comment.accept_mut(visitor),
        }
    }
}
