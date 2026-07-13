use std::str::Chars;

use chumsky::{Parser, extra::{self, ParserExtra}, input::Stream, prelude::{custom, todo}, span::{SimpleSpan, Spanned}};

use crate::data::error::Error;

fn span_convert<T: Iterator<Item = usize> + Clone>(
    sizes: &T,
    mut target: usize,
) -> Option<(usize, usize)> {
    for (i, size) in sizes.clone().enumerate() {
        if target > size {
            target -= size;
        } else {
            return Some((i, target));
        }
    }

    None
}

pub fn composite_text<T>(
    data: Vec<Spanned<String>>,
    parser: impl for<'src> Parser<'src, &'src str, T, extra::Err<Error>>,
) -> (Option<T>, Vec<Error>) {
    let sizes = data.iter().map(|data_str| data_str.inner.len());
    let data_str = data
        .iter()
        .map(|data_str| data_str.inner.clone())
        .collect::<Vec<_>>()
        .join("");

    parser
        .map_err(|mut err| {
            err.span.start = span_convert(&sizes, err.span.start)
                .map(|(i, len)| data[i].span.start + len)
                .unwrap();
            err.span.end = span_convert(&sizes, err.span.end)
                .map(|(i, len)| data[i].span.start + len)
                .unwrap();
            err
        })
        .parse(data_str.as_str())
        .into_output_errors()
}

pub fn composite_text_p<'src, T, E: ParserExtra<'src, Stream<Chars<'src>>>>(
    spans_parser: impl Parser<'src, Stream<Chars<'src>>, Vec<SimpleSpan>, E>,
    base_parser: impl Parser<'src, Stream<Chars<'src>>, T, E>,
) -> impl Parser<'src, Stream<Chars<'src>>, (Option<T>, Vec<E::Error>), E> {
    todo()
}