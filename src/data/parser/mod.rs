pub mod comments;
pub mod composite_text;
pub mod enumerate;
pub mod formula;
pub mod paragraph;
pub mod params;
pub mod text;
pub mod title;

use crate::data::{
    ParseData,
    error::Error,
    parser::{
        comments::comments, enumerate::enumerate, formula::formula, paragraph::paragraph,
        title::title,
    },
};
use chumsky::{
    IterParser, Parser, extra,
    prelude::{choice, just, recursive},
};

pub fn strict_mark<'src>() -> impl Parser<'src, &'src str, ParseData, extra::Err<Error>> {
    recursive(|strict_mark| {
        choice((
            just("\n").to(ParseData::PhantomNewLine),
            title(),
            paragraph(),
            formula(),
            comments(),
            //enumerate(strict_mark),
        ))
        .repeated()
        .collect()
        .map(ParseData::List)
    })
}
