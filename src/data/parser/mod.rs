pub mod comments;
pub mod enumerate;
pub mod formula;
pub mod paragraph;
pub mod params;
pub mod text;
pub mod title;

use crate::data::{
    PreParseData,
    error::Error,
    parser::{
        comments::comments, enumerate::enumerate, formula::formula, paragraph::paragraph, title::title
    },
};
use chumsky::{
    IterParser, Parser, extra,
    prelude::{choice, just},
};

pub fn strict_mark<'src>() -> impl Parser<'src, &'src str, PreParseData, extra::Err<Error>> {
    choice((
        just("\n").to(PreParseData::PhantomNewLine),
        title(),
        formula(),
        comments(),
        enumerate(),
        paragraph(),
    ))
    .repeated()
    .collect()
    .map(PreParseData::List)
}
