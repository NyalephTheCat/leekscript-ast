use global_declaration::GlobalDeclaration;
use nom::{branch::alt, combinator::map, sequence::pair, IResult};
use variable_declaration::VariableDeclaration;

use crate::{
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::utils::flags::{Flag, HasFlag, WithFlag};

pub mod assignment;
pub mod global_declaration;
pub mod variable_declaration;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GlobalFlag;
impl Flag for GlobalFlag {
    const FLAG: bool = true;
}
impl WithFlag<GlobalFlag> for GlobalFlag {
    const HAS: bool = true;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statements<F: WithFlag<GlobalFlag>> {
    GlobalDeclaration(HasFlag<GlobalFlag, F>, GlobalDeclaration),
    VariableDeclaration(VariableDeclaration),
}

impl<'a, F: WithFlag<GlobalFlag>> Parser<&'a str> for Statements<F> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            map(
                pair(<_ as Parser<&'a str>>::parse, <_ as Parser<&'a str>>::parse),
                |(first, last)| Self::GlobalDeclaration(first, last),
            ),
            map(<_ as Parser<&'a str>>::parse, Self::VariableDeclaration),
        ))(input)
    }
}
impl<V: Visitor, F: WithFlag<GlobalFlag>> Visitable<V> for Statements<F> {
    default fn accept(&self, v: &mut V) {
        match self {
            Self::GlobalDeclaration(flag, node) => {
                flag.accept(v);
                node.accept(v);
            }
            Self::VariableDeclaration(node) => node.accept(v),
        }
    }
}
impl<V: VisitorMut, F: WithFlag<GlobalFlag>> VisitableMut<V> for Statements<F> {
    default fn accept_mut(&mut self, v: &mut V) {
        match self {
            Self::GlobalDeclaration(flag, node) => {
                flag.accept_mut(v);
                node.accept_mut(v);
            }
            Self::VariableDeclaration(node) => node.accept_mut(v),
        }
    }
}
