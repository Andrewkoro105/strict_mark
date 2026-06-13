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
    parser::{comments::parser_comments, formula::parser_formula, paragraph::parser_paragraph, title::parser_title},
};

pub fn strict_mark_parser<'src>() -> impl Parser<'src, &'src str, Vec<ParseData>, extra::Err<Error<'src>>>
{
    recursive(|strict_mark_parser| {
        choice((
            just("\n").to(ParseData::PhantomNewLine),
            parser_title(),
            parser_paragraph(),
            parser_formula(),
            parser_comments(),
        ))
        .repeated()
        .collect()
    })
}
