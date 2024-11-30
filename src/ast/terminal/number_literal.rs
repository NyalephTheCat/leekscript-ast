use crate::{
    parser::Parser,
    visitor::{writer::Writer, Visitable, VisitableMut, Visitor, VisitorMut},
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{map, opt, recognize},
    sequence::tuple,
    IResult,
};

/// Represents a parsed number literal with its kind and original content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberLiteral {
    pub kind: NumberKind,
    pub content: String,
}

impl<'a> Parser<&'a str> for NumberLiteral {
    fn parse(input: &'a str) -> IResult<&'a str, Self> {
        alt((
            map(parse_hex, |s: &str| NumberLiteral {
                kind: NumberKind::Hex,
                content: s.to_string(),
            }),
            map(parse_octal, |s: &str| NumberLiteral {
                kind: NumberKind::Octal,
                content: s.to_string(),
            }),
            map(parse_binary, |s: &str| NumberLiteral {
                kind: NumberKind::Binary,
                content: s.to_string(),
            }),
            map(parse_float, |s: &str| NumberLiteral {
                kind: NumberKind::Float,
                content: s.to_string(),
            }),
            map(parse_decimal, |s: &str| NumberLiteral {
                kind: NumberKind::Decimal,
                content: s.to_string(),
            }),
        ))(input)
    }
}

/// Enumerates the different kinds of number literals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberKind {
    Hex,
    Octal,
    Binary,
    Float,
    Decimal,
}

impl<V: Visitor> Visitable<V> for NumberKind {
    default fn accept(&self, _: &mut V) {}
}

impl<V: VisitorMut> VisitableMut<V> for NumberKind {
    default fn accept_mut(&mut self, _: &mut V) {}
}

impl Visitable<Writer> for NumberLiteral {
    fn accept(&self, v: &mut Writer) {
        v.0 += &self.content;
    }
}

/// Parses a hexadecimal number (e.g., `0xfff`).
fn parse_hex(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        alt((tag("0x"), tag("0X"))),
        take_while1(|c: char| c.is_digit(16) || c == '_'),
    )))(input)
}

/// Parses an octal number (e.g., `0o755`).
fn parse_octal(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        alt((tag("0o"), tag("0O"))),
        take_while1(|c: char| ('0'..='7').contains(&c) || c == '_'),
    )))(input)
}

/// Parses a binary number (e.g., `0b1010`).
fn parse_binary(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        alt((tag("0b"), tag("0B"))),
        take_while1(|c: char| c == '0' || c == '1' || c == '_'),
    )))(input)
}

/// Parses a floating-point number, including those with exponents (e.g., `123.456`, `1e10`).
fn parse_float(input: &str) -> IResult<&str, &str> {
    alt((
        // e.g., 123.456e+78
        recognize(tuple((
            take_while1(|c: char| c.is_digit(10) || c == '_'),
            tag("."),
            take_while1(|c: char| c.is_digit(10) || c == '_'),
            alt((tag("e"), tag("E"))),
            opt(alt((tag("+"), tag("-")))),
            take_while1(|c: char| c.is_digit(10) || c == '_'),
        ))),
        // e.g., 1e10
        recognize(tuple((
            take_while1(|c: char| c.is_digit(10) || c == '_'),
            alt((tag("e"), tag("E"))),
            opt(alt((tag("+"), tag("-")))),
            take_while1(|c: char| c.is_digit(10) || c == '_'),
        ))),
        // e.g., .123
        recognize(tuple((
            tag("."),
            take_while1(|c: char| c.is_digit(10) || c == '_'),
        ))),
        // e.g., 123.
        recognize(tuple((
            take_while1(|c: char| c.is_digit(10) || c == '_'),
            tag("."),
        ))),
    ))(input)
}

/// Parses a decimal number (e.g., `123`, `1_234_567`).
fn parse_decimal(input: &str) -> IResult<&str, &str> {
    recognize(take_while1(|c: char| c.is_digit(10) || c == '_'))(input)
}

impl<V: Visitor> Visitable<V> for NumberLiteral {
    default fn accept(&self, v: &mut V) {
        v.visit(&self.kind);
    }
}

impl<V: VisitorMut> VisitableMut<V> for NumberLiteral {
    default fn accept_mut(&mut self, v: &mut V) {
        self.kind.accept_mut(v);
    }
}
