pub mod comments;
pub mod formula;
pub mod paragraph;
pub mod params;
pub mod text;
pub mod title;

use chumsky::{
    IterParser, Parser, extra, prelude::{choice, just, recursive, todo}
};

use crate::data::{
    ParagraphType, ParseData, TextVariants,
    error::Error,
    parser::{comments::comments, formula::formula, paragraph::paragraph, title::title},
};

pub fn strict_mark<'src>() -> impl Parser<'src, &'src str, Vec<ParseData>, extra::Err<Error<'src>>>
{
    recursive(|strict_mark_parser| {
        choice((
            just("\n").to(ParseData::PhantomNewLine),
            title(),
            paragraph(),
            formula(),
            comments(),
        ))
        .repeated()
        .collect()
    })
}
