use core::fmt;
use std::fmt::Formatter;

use nom::{combinator::map, sequence::tuple};

use crate::{
    ast::utils::repeat::Repeat0,
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::trivia::Trivia;

#[derive(Clone, PartialEq, Eq)]
pub struct WithTrivia<T> {
    pub trivias: Vec<Trivia>,
    pub node: T,
}

impl<'a, T: Parser<&'a str>> Parser<&'a str> for WithTrivia<T> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        map(
            tuple((Repeat0::<Trivia>::parse, T::parse)),
            |(trivias, node)| Self {
                trivias: trivias.0,
                node,
            },
        )(input)
    }
}

impl<V: Visitor, T: Visitable<V>> Visitable<V> for WithTrivia<T> {
    default fn accept(&self, visitor: &mut V) {
        visitor.visit(&self.trivias);
        visitor.visit(&self.node);
    }
}

impl<V: VisitorMut, T: VisitableMut<V>> VisitableMut<V> for WithTrivia<T> {
    default fn accept_mut(&mut self, visitor: &mut V) {
        self.trivias.accept_mut(visitor);
        self.node.accept_mut(visitor);
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for WithTrivia<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // If there isn't any trivia, we want to print the node inside directly using the debug
        // builder
        if self.trivias.is_empty() {
            f.debug_tuple("WithTrivia").field(&self.node).finish()
        } else {
            f.debug_tuple("WithTrivia")
                .field(&self.trivias)
                .field(&self.node)
                .finish()
        }
    }
}
