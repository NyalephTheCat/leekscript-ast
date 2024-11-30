use nom::{combinator::map, sequence::tuple, IResult};

use crate::{
    ast::{
        terminal::{
            keyword::KwInclude,
            string_literal::StringLiteral,
            symbol::{LParen, RParen, Semi},
        },
        trivia::with_trivia::WithTrivia,
    },
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeStatement {
    pub kw_include: WithTrivia<KwInclude>,
    pub lparen: WithTrivia<LParen>,
    pub path: WithTrivia<StringLiteral>,
    pub rparen: WithTrivia<RParen>,
    pub semi: Option<WithTrivia<Semi>>,
}

impl<'a> Parser<&'a str> for IncludeStatement {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
            )),
            |(kw_include, lparen, path, rparen, semi)| Self {
                kw_include,
                lparen,
                path,
                rparen,
                semi,
            },
        )(input)
    }
}

impl<V: Visitor> Visitable<V> for IncludeStatement {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.kw_include);
        v.visit(&self.lparen);
        v.visit(&self.path);
        v.visit(&self.rparen);
        v.visit(&self.semi);
    }
}

impl<V: VisitorMut> VisitableMut<V> for IncludeStatement {
    default fn accept_mut(&mut self, v: &mut V) {
        self.kw_include.accept_mut(v);
        self.lparen.accept_mut(v);
        self.path.accept_mut(v);
        self.rparen.accept_mut(v);
        self.semi.accept_mut(v);
    }
}
