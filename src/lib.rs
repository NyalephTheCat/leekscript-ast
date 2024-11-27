#![feature(associated_const_equality)]
#![feature(specialization)]
#![feature(associated_type_defaults)]

use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, satisfy},
    combinator::{map, opt, recognize, verify},
    multi::many0,
    sequence::tuple,
};
use parser::Parser;
use serializer::Writer;
use visitor::{Visitable, VisitableMut, Visitor, VisitorMut};

pub mod parser;
pub mod visitor;

pub mod serializer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Whitespace(pub String);

impl<'a> Parser<&'a str> for Whitespace {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        map(nom::character::complete::multispace1, |ws: &'a str| {
            Self(ws.into())
        })(input)
    }
}

impl<V: Visitor> Visitable<V> for Whitespace {
    default fn accept(&self, _: &mut V) {}
}

impl<V: VisitorMut> VisitableMut<V> for Whitespace {
    default fn accept_mut(&mut self, _: &mut V) {}
}

impl Visitable<Writer> for Whitespace {
    fn accept(&self, visitor: &mut Writer) {
        visitor.0 += self.0.as_str();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comment {
    SingleLine(String),
    MultiLine(String),
}

impl<'a> Parser<&'a str> for Comment {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        alt((
            map(
                tuple((tag("//"), take_until("\n"), opt(tag("\n")))),
                |(_, content, _): (_, &'a str, _)| Self::SingleLine(content.into()),
            ),
            map(
                tuple((tag("/*"), take_until("*/"), tag("*/"))),
                |(_, content, _): (_, &'a str, _)| Self::MultiLine(content.into()),
            ),
        ))(input)
    }
}

impl<V: Visitor> Visitable<V> for Comment {
    default fn accept(&self, _: &mut V) {}
}

impl<V: VisitorMut> VisitableMut<V> for Comment {
    default fn accept_mut(&mut self, _: &mut V) {}
}

impl Visitable<Writer> for Comment {
    fn accept(&self, visitor: &mut Writer) {
        match self {
            Comment::SingleLine(s) => {
                visitor.0 += format!("//{}", s).as_str();
            }
            Comment::MultiLine(s) => {
                visitor.0 += format!("/*{}*/", s).as_str();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trivia {
    Whitespace(Whitespace),
    Comment(Comment),
}

impl<'a> Parser<&'a str> for Trivia {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        alt((
            map(Whitespace::parse, Self::Whitespace),
            map(Comment::parse, Self::Comment),
        ))(input)
    }
}

impl<V: Visitor> Visitable<V> for Trivia {
    default fn accept(&self, visitor: &mut V) {
        match self {
            Trivia::Comment(c) => visitor.visit(c),
            Trivia::Whitespace(s) => visitor.visit(s),
        }
    }
}

impl<V: VisitorMut> VisitableMut<V> for Trivia {
    default fn accept_mut(&mut self, visitor: &mut V) {
        match self {
            Trivia::Comment(c) => visitor.visit_mut(c),
            Trivia::Whitespace(s) => visitor.visit_mut(s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithTrivia<T>(pub Vec<Trivia>, pub T);

impl<'a, T: Parser<&'a str>> Parser<&'a str> for WithTrivia<T> {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        map(
            tuple((parser::repeat::Repeat0::<Trivia>::parse, T::parse)),
            |(trivia, item)| Self(trivia.into(), item),
        )(input)
    }
}

impl<V: Visitor, T: Visitable<V>> Visitable<V> for WithTrivia<T> {
    fn accept(&self, visitor: &mut V) {
        self.0.accept(visitor);
        self.1.accept(visitor);
    }
}

impl<V: VisitorMut, T: VisitableMut<V>> VisitableMut<V> for WithTrivia<T> {
    default fn accept_mut(&mut self, visitor: &mut V) {
        self.0.accept_mut(visitor);
        self.1.accept_mut(visitor);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier(pub String);

impl<'a> Parser<&'a str> for Identifier {
    fn parse(input: &'a str) -> nom::IResult<&'a str, Self> {
        // List of reserved keywords
        const KEYWORDS: &[&str] = &[
            "if", "else", "for", "while", "do", // Add more keywords as needed
        ];

        // Parser for the first character: alphabetic or '_'
        let first_char = alt((alpha1, tag("_")));

        // Parser for subsequent characters: alphanumeric or '_'
        let rest_chars = recognize(tuple((
            // We use `take_while` to consume zero or more valid characters
            // Since `alphanumeric1` requires at least one, we define our own
            // Using a closure to check the character
            many0(satisfy(|c| c.is_alphanumeric() || c == '_')),
        )));

        // Combine first and rest
        let identifier_parser = recognize(tuple((first_char, rest_chars)));

        // Use `verify` to ensure the parsed identifier is not a keyword
        let mut valid_identifier =
            verify(identifier_parser, |ident: &str| !KEYWORDS.contains(&ident));

        // Parse the identifier
        let (remaining_input, ident_str) = valid_identifier(input)?;

        // Return the parsed Identifier
        Ok((remaining_input, Identifier(ident_str.to_string())))
    }
}

impl<V: Visitor> Visitable<V> for Identifier {
    default fn accept(&self, _: &mut V) {}
}

impl<V: VisitorMut> VisitableMut<V> for Identifier {
    default fn accept_mut(&mut self, _: &mut V) {}
}

impl Visitable<Writer> for Identifier {
    fn accept(&self, visitor: &mut Writer) {
        visitor.0 += self.0.as_str();
    }
}
