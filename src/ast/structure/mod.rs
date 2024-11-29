use nom::{
    branch::alt,
    combinator::map,
    sequence::{pair, tuple},
    IResult,
};

use crate::{
    parser::Parser,
    visitor::{Visitable, VisitableMut, Visitor, VisitorMut},
};

use super::{expressions::Expression, terminal::symbol::Equal};
use super::{
    terminal::{identifier::Identifier, symbol::Comma},
    trivia::with_trivia::WithTrivia,
    utils::separated::Separated1,
};

pub mod eof;
pub mod file;
pub mod type_struct;
