use nom::{combinator::map, sequence::tuple, IResult};

use crate::{
    ast::{
        statements::assignment::AssignmentList,
        structure::type_struct::TypeFollowedById,
        terminal::{keyword::KwGlobal, symbol::Semi},
        trivia::with_trivia::WithTrivia,
    },
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

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
