pub mod comments;
pub mod enumerate;
pub mod formula;
pub mod paragraph;
pub mod params;
pub mod text;
pub mod title;
pub mod insertion;

use crate::data::{
    PreParseData,
    error::Error,
    parser::{
        comments::comments, enumerate::enumerate, formula::formula, insertion::resource_insertion, paragraph::paragraph, title::title
    },
};
use chumsky::{
    IterParser, Parser, extra,
    prelude::{choice, just},
};

pub fn strict_mark<'src>() -> impl Parser<'src, &'src str, PreParseData, extra::Err<Error>> {
    choice((
        just("\n").to(PreParseData::PhantomNewLine),
        comments(),
        title(),
        formula(),
        resource_insertion(),
        enumerate(),
        paragraph(),
    ))
    .repeated()
    .collect()
    .map(PreParseData::List)
}
