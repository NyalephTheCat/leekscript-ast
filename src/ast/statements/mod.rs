use global_declaration::GlobalDeclaration;
use if_statement::IfStatement;
use include_statement::IncludeStatement;
use nom::{branch::alt, combinator::map, sequence::pair, IResult};
use variable_declaration::VariableDeclaration;

use crate::{
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::{
    expressions::Expression,
    structure::file::File,
    terminal::symbol::Semi,
    trivia::{trivia::Trivia, with_trivia::WithTrivia},
    utils::flags::{Flag, HasFlag, WithFlag},
};

pub mod assignment;
pub mod global_declaration;
pub mod if_statement;
pub mod include_statement;
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
    IncludeStatement(HasFlag<GlobalFlag, F>, IncludeStatement),
    // ClassDeclaration(HasFlag<GlobalFlag, F>, ClassDeclaration),
    VariableDeclaration(VariableDeclaration),
    IfStatement(IfStatement),
    SemiColon(WithTrivia<Semi>),
    Expression(Expression),

    // Meta - Not parsed but used as transformation outputs
    IncludedFile(Box<File>),
    EmptyStatement(Vec<Trivia>), // CAUTION, this one should never be parsed manually
}

impl<'a, F: WithFlag<GlobalFlag>> Parser<&'a str> for Statements<F> {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            map(
                pair(<_ as Parser<&'a str>>::parse, <_ as Parser<&'a str>>::parse),
                |(first, last)| Self::GlobalDeclaration(first, last),
            ),
            map(
                pair(<_ as Parser<&'a str>>::parse, <_ as Parser<&'a str>>::parse),
                |(first, last)| Self::IncludeStatement(first, last),
            ),
            map(<_ as Parser<&'a str>>::parse, Self::VariableDeclaration),
            map(<_ as Parser<&'a str>>::parse, Self::IfStatement),
            map(<_ as Parser<&'a str>>::parse, Self::Expression),
            map(<_ as Parser<&'a str>>::parse, Self::SemiColon),
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
            Self::IncludeStatement(flag, node) => {
                flag.accept(v);
                node.accept(v);
            }
            Self::VariableDeclaration(node) => node.accept(v),
            Self::IfStatement(node) => node.accept(v),
            Self::Expression(node) => node.accept(v),
            Self::IncludedFile(file) => file.accept(v),
            Self::SemiColon(node) => v.visit(node),
            Self::EmptyStatement(trivia) => v.visit(trivia),
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
            Self::IncludeStatement(flag, node) => {
                flag.accept_mut(v);
                node.accept_mut(v);
            }
            Self::VariableDeclaration(node) => node.accept_mut(v),
            Self::IfStatement(node) => node.accept_mut(v),
            Self::IncludedFile(file) => file.accept_mut(v),
            Self::SemiColon(node) => node.accept_mut(v),
            Self::Expression(node) => node.accept_mut(v),
            Self::EmptyStatement(trivia) => trivia.accept_mut(v),
        }
    }
}
