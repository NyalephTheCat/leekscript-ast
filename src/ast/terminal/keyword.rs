use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    combinator::{map, not, peek},
    sequence::terminated,
};

use crate::{
    parser::Parser,
    visitor::{writer::Writer, Visitable, VisitableMut, Visitor, VisitorMut},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keywords {
    Global(KwGlobal),
    Var(KwVar),
    Include(KwInclude),
}

impl<'a> Parser<&'a str> for Keywords {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        alt((
            map(KwGlobal::parse, Keywords::Global),
            map(KwVar::parse, Keywords::Var),
        ))(input)
    }
}

impl<V: Visitor> Visitable<V> for Keywords {
    default fn accept(&self, v: &mut V) {
        match self {
            Keywords::Global(node) => v.visit(node),
            Keywords::Var(node) => v.visit(node),
            Keywords::Include(node) => v.visit(node),
        }
    }
}

impl<V: VisitorMut> VisitableMut<V> for Keywords {
    default fn accept_mut(&mut self, v: &mut V) {
        match self {
            Keywords::Global(node) => node.accept_mut(v),
            Keywords::Var(node) => node.accept_mut(v),
            Keywords::Include(node) => node.accept_mut(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KwGlobal;
impl<'a> Parser<&'a str> for KwGlobal {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        map(
            terminated(tag("global"), peek(not(alt((alphanumeric1, tag("_")))))),
            |_| KwGlobal,
        )(input)
    }
}

impl<V: Visitor> Visitable<V> for KwGlobal {
    default fn accept(&self, _: &mut V) {}
}

impl<V: VisitorMut> VisitableMut<V> for KwGlobal {
    default fn accept_mut(&mut self, _: &mut V) {}
}

impl Visitable<Writer> for KwGlobal {
    fn accept(&self, v: &mut Writer) {
        v.0 += "global";
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KwVar;
impl<'a> Parser<&'a str> for KwVar {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        map(
            terminated(tag("var"), peek(not(alt((alphanumeric1, tag("_")))))),
            |_| KwVar,
        )(input)
    }
}

impl<V: Visitor> Visitable<V> for KwVar {
    default fn accept(&self, _: &mut V) {}
}

impl<V: VisitorMut> VisitableMut<V> for KwVar {
    default fn accept_mut(&mut self, _: &mut V) {}
}

impl Visitable<Writer> for KwVar {
    fn accept(&self, v: &mut Writer) {
        v.0 += "var";
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KwInclude;
impl<'a> Parser<&'a str> for KwInclude {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        map(
            terminated(tag("include"), peek(not(alt((alphanumeric1, tag("_")))))),
            |_| KwInclude,
        )(input)
    }
}

impl<V: Visitor> Visitable<V> for KwInclude {
    default fn accept(&self, _: &mut V) {}
}

impl<V: VisitorMut> VisitableMut<V> for KwInclude {
    default fn accept_mut(&mut self, _: &mut V) {}
}

impl Visitable<Writer> for KwInclude {
    fn accept(&self, v: &mut Writer) {
        v.0 += "include";
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KwIf;
impl<'a> Parser<&'a str> for KwIf {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        map(
            terminated(tag("if"), peek(not(alt((alphanumeric1, tag("_")))))),
            |_| KwIf,
        )(input)
    }
}

impl<V: Visitor> Visitable<V> for KwIf {
    default fn accept(&self, _: &mut V) {}
}
impl<V: VisitorMut> VisitableMut<V> for KwIf {
    default fn accept_mut(&mut self, _: &mut V) {}
}
impl Visitable<Writer> for KwIf {
    fn accept(&self, v: &mut Writer) {
        v.0 += "if";
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KwElse;
impl<'a> Parser<&'a str> for KwElse {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        map(
            terminated(tag("else"), peek(not(alt((alphanumeric1, tag("_")))))),
            |_| KwElse,
        )(input)
    }
}
impl<V: Visitor> Visitable<V> for KwElse {
    default fn accept(&self, _: &mut V) {}
}
impl<V: VisitorMut> VisitableMut<V> for KwElse {
    default fn accept_mut(&mut self, _: &mut V) {}
}
impl Visitable<Writer> for KwElse {
    fn accept(&self, v: &mut Writer) {
        v.0 += "else";
    }
}
