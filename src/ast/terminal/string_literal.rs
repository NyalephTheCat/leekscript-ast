use nom::{
    branch::alt, bytes::complete::take_until, combinator::map, sequence::delimited, IResult,
};

use crate::{
    parser::Parser,
    visitor::{writer::Writer, Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::symbol::{DQuote, SQuote};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral {
    pub quote: Quote,
    pub content: String,
}

impl<'a> Parser<&'a str> for StringLiteral {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            map(
                delimited(SQuote::parse, take_until("'"), SQuote::parse),
                |string| Self {
                    quote: Quote::Single(SQuote),
                    content: string.to_string(),
                },
            ),
            map(
                delimited(DQuote::parse, take_until("\""), DQuote::parse),
                |string| Self {
                    quote: Quote::Double(DQuote),
                    content: string.to_string(),
                },
            ),
        ))(input)
    }
}

impl<V: Visitor> Visitable<V> for StringLiteral {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.quote);
    }
}

impl<V: VisitorMut> VisitableMut<V> for StringLiteral {
    default fn accept_mut(&mut self, v: &mut V) {
        self.quote.accept_mut(v);
    }
}

impl Visitable<Writer> for StringLiteral {
    fn accept(&self, v: &mut Writer) {
        self.quote.accept(v);
        v.0 += &self.content;
        self.quote.accept(v);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quote {
    Single(SQuote),
    Double(DQuote),
}

impl<'a> Parser<&'a str> for Quote {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            map(SQuote::parse, Quote::Single),
            map(DQuote::parse, Quote::Double),
        ))(input)
    }
}

impl<V: Visitor> Visitable<V> for Quote {
    default fn accept(&self, v: &mut V) {
        match self {
            Quote::Single(node) => v.visit(node),
            Quote::Double(node) => v.visit(node),
        }
    }
}

impl<V: VisitorMut> VisitableMut<V> for Quote {
    default fn accept_mut(&mut self, v: &mut V) {
        match self {
            Quote::Single(node) => node.accept_mut(v),
            Quote::Double(node) => node.accept_mut(v),
        }
    }
}
