use nom::{combinator::map, sequence::tuple, IResult};

use crate::{
    ast::{
        expressions::Expression,
        terminal::{
            keyword::{KwElse, KwIf},
            symbol::{LParen, RParen},
        },
        trivia::with_trivia::WithTrivia,
        utils::flags::Nil,
    },
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::Statements;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStatement {
    pub if_kw: WithTrivia<KwIf>,
    pub lparen: WithTrivia<LParen>,
    pub condition: WithTrivia<Expression>,
    pub rparen: WithTrivia<RParen>,
    pub body: WithTrivia<Box<Statements<Nil>>>,
    pub else_clause: Option<WithTrivia<ElseClause>>,
}

impl<'a> Parser<&'a str> for IfStatement {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
                <_ as Parser<&'a str>>::parse,
            )),
            |(if_kw, lparen, condition, rparen, body, else_clause)| Self {
                if_kw,
                lparen,
                condition,
                rparen,
                body,
                else_clause,
            },
        )(input)
    }
}

impl<V: Visitor> Visitable<V> for IfStatement {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.if_kw);
        v.visit(&self.lparen);
        v.visit(&self.condition);
        v.visit(&self.rparen);
        v.visit(&self.body);
        v.visit(&self.else_clause);
    }
}

impl<V: VisitorMut> VisitableMut<V> for IfStatement {
    default fn accept_mut(&mut self, v: &mut V) {
        v.visit_mut(&mut self.if_kw);
        v.visit_mut(&mut self.lparen);
        v.visit_mut(&mut self.condition);
        v.visit_mut(&mut self.rparen);
        v.visit_mut(&mut self.body);
        v.visit_mut(&mut self.else_clause);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseClause {
    pub else_kw: WithTrivia<KwElse>,
    pub body: WithTrivia<Box<Statements<Nil>>>,
}

impl<'a> Parser<&'a str> for ElseClause {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        map(
            tuple((<_ as Parser<&'a str>>::parse, <_ as Parser<&'a str>>::parse)),
            |(else_kw, body)| Self { else_kw, body },
        )(input)
    }
}

impl<V: Visitor> Visitable<V> for ElseClause {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.else_kw);
        v.visit(&self.body);
    }
}

impl<V: VisitorMut> VisitableMut<V> for ElseClause {
    default fn accept_mut(&mut self, v: &mut V) {
        v.visit_mut(&mut self.else_kw);
        v.visit_mut(&mut self.body);
    }
}
