pub mod comments;
pub mod formula;
pub mod paragraph;
pub mod params;
pub mod text;
pub mod title;

use chumsky::{Parser, prelude::todo};

use crate::data::ParsData;

pub fn strict_mark_parser<'src>() -> impl Parser<'src, &'src str, ParsData> {
    todo()
}
