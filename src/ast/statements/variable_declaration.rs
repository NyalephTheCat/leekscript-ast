use nom::{branch::alt, combinator::map, sequence::tuple, IResult};

use crate::{
    ast::{
        statements::assignment::AssignmentList,
        structure::type_struct::TypeFollowedById,
        terminal::{keyword::KwVar, symbol::Semi},
        trivia::with_trivia::WithTrivia,
    },
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

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
