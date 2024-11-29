use nom::{bytes::complete::tag, combinator::map, IResult};

use crate::{
    parser::Parser,
    visitor::{writer::Writer, Visitable, VisitableMut, Visitor, VisitorMut},
};

macro_rules! define_symbol {
    // Match a list of punctuations: Name => "symbol"
    ( $( $name:ident => $symbol:expr ),* )
    => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $name;

            impl<'a> Parser<&'a str> for $name {
                fn parse(input: &'a str) -> IResult<&'a str, Self> {
                    map(tag($symbol), |_| $name)(input)
                }
            }

            impl<V: Visitor> Visitable<V> for $name {
                default fn accept(&self, _: &mut V) {}
            }

            impl<V: VisitorMut> VisitableMut<V> for $name {
                default fn accept_mut(&mut self, _: &mut V) {}
            }

            impl Visitable<Writer> for $name {
                fn accept(&self, visitor: &mut Writer) {
                    visitor.0 += $symbol;
                }
            }
        )*
    };
}

define_symbol! {
    Dot => ".",
    Semi => ";",
    Comma => ",",
    Colon => ":",
    Equal => "=",
    // Operators
    Plus => "+",
    Minus => "-",
    Star => "*",
    Slash => "/",
    Percent => "%",
    Backslash => "\\",
    Ampersand => "&",
    Pipe => "|",
    Caret => "^",
    Tilde => "~",
    Exclamation => "!",
    Question => "?",
    Underscore => "_",
    // Quotes
    Backtick => "`",
    SQuote => "'",
    DQuote => "\"",
    // Brackets
    LAngle => "<",
    RAngle => ">",
    LBracket => "[",
    RBracket => "]",
    LParen => "(",
    RParen => ")",
    LBrace => "{",
    RBrace => "}"
}
