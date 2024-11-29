use nom::{
    branch::alt,
    combinator::map,
    sequence::{pair, tuple},
    IResult,
};
use type_struct::TypeFollowedById;

use crate::{
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::{
    terminal::{
        identifier::Identifier,
        keyword::{KwGlobal, KwVar},
        symbol::{Comma, Equal, Semi},
    },
    trivia::with_trivia::WithTrivia,
    utils::{
        flags::{Flag, HasFlag, WithFlag},
        separated::Separated1,
    },
};

pub mod type_struct;

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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GlobalFlag;
impl Flag for GlobalFlag {
    const FLAG: bool = true;
}
impl WithFlag<GlobalFlag> for GlobalFlag {
    const HAS: bool = true;
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalDeclaration(
    pub WithTrivia<KwGlobal>,
    pub Option<WithTrivia<TypeFollowedById>>,
    pub AssignmentList,
    pub Option<WithTrivia<Semi>>,
);
impl<'a> Parser<&'a str> for GlobalDeclaration {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
            )),
            |(global, ty, assignments, semi)| Self(global, ty, assignments, semi),
        )(input)
    }
}
impl<V: Visitor> Visitable<V> for GlobalDeclaration {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.0);
        v.visit(&self.1);
        v.visit(&self.2);
        v.visit(&self.3);
    }
}
impl<V: VisitorMut> VisitableMut<V> for GlobalDeclaration {
    default fn accept_mut(&mut self, v: &mut V) {
        self.0.accept_mut(v);
        self.1.accept_mut(v);
        self.2.accept_mut(v);
        self.3.accept_mut(v);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableDeclaration {
    WithType(TypeFollowedById, AssignmentList, Option<WithTrivia<Semi>>),
    UnspecifiedType(KwVar, AssignmentList, Option<WithTrivia<Semi>>),
}

impl<'a> Parser<&'a str> for VariableDeclaration {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            map(
                tuple((
                    <_ as Parser<&'a str>>::parse,
                    <_ as Parser<&'a str>>::parse,
                    <_ as Parser<&'a str>>::parse,
                )),
                |(ty, assignments, semi)| Self::WithType(ty, assignments, semi),
            ),
            map(
                tuple((
                    <_ as Parser<&'a str>>::parse,
                    <_ as Parser<&'a str>>::parse,
                    <_ as Parser<&'a str>>::parse,
                )),
                |(var, assignments, semi)| Self::UnspecifiedType(var, assignments, semi),
            ),
        ))(input)
    }
}

impl<V: Visitor> Visitable<V> for VariableDeclaration {
    default fn accept(&self, v: &mut V) {
        match self {
            Self::WithType(ty, assignments, semi) => {
                v.visit(ty);
                v.visit(assignments);
                v.visit(semi);
            }
            Self::UnspecifiedType(var, assignments, semi) => {
                v.visit(var);
                v.visit(assignments);
                v.visit(semi);
            }
        }
    }
}

impl<V: VisitorMut> VisitableMut<V> for VariableDeclaration {
    default fn accept_mut(&mut self, v: &mut V) {
        match self {
            Self::WithType(ty, assignments, semi) => {
                ty.accept_mut(v);
                assignments.accept_mut(v);
                semi.accept_mut(v);
            }
            Self::UnspecifiedType(var, assignments, semi) => {
                var.accept_mut(v);
                assignments.accept_mut(v);
                semi.accept_mut(v);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentList(pub Separated1<WithTrivia<Assignment>, WithTrivia<Comma>>);
impl<'a> Parser<&'a str> for AssignmentList {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(<_ as Parser<&'a str>>::parse, Self)(input)
    }
}
impl<V: Visitor> Visitable<V> for AssignmentList {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.0);
    }
}
impl<V: VisitorMut> VisitableMut<V> for AssignmentList {
    default fn accept_mut(&mut self, v: &mut V) {
        self.0.accept_mut(v);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment(
    Identifier,
    Option<(WithTrivia<Equal>, WithTrivia<Expression>)>,
);
impl<'a> Parser<&'a str> for Assignment {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            pair(<_ as Parser<&'a str>>::parse, <_ as Parser<&'a str>>::parse),
            |(first, last)| Self(first, last),
        )(input)
    }
}
impl<V: Visitor> Visitable<V> for Assignment {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.0);
        v.visit(&self.1);
    }
}
impl<V: VisitorMut> VisitableMut<V> for Assignment {
    default fn accept_mut(&mut self, v: &mut V) {
        self.0.accept_mut(v);
        self.1.accept_mut(v);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
}
impl<'a> Parser<&'a str> for Expression {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(<_ as Parser<&'a str>>::parse, Self::Identifier)(input)
    }
}
impl<V: Visitor> Visitable<V> for Expression {
    default fn accept(&self, v: &mut V) {
        match self {
            Self::Identifier(node) => v.visit(node),
        }
    }
}
impl<V: VisitorMut> VisitableMut<V> for Expression {
    default fn accept_mut(&mut self, v: &mut V) {
        match self {
            Self::Identifier(node) => node.accept_mut(v),
        }
    }
}
