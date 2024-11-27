use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, satisfy},
    combinator::{recognize, verify},
    multi::many0,
    sequence::tuple,
};

use crate::{
    parser::Parser,
    visitor::{writer::Writer, Visitable, VisitableMut, Visitor, VisitorMut},
};

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
