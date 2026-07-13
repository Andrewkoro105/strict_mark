use chumsky::prelude::*;

use crate::data::{
    ParseData, TextVariants,
    error::{Block, Error, Expected}, parser::composite_text::composite_text,
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EnumerateExpected {
    Delimiter(usize),
    Text,
}

pub fn enumerate<'src>(
    strict_mark: impl for<'inner_src> Parser<'inner_src, &'inner_src str, ParseData, extra::Err<Error>> + Clone + 'src,
) -> impl Parser<'src, &'src str, ParseData, extra::Err<Error>> + Clone {
    just("- ")
        .ignore_then(
            any()
                .to_slice()
                .and_is(just("\n").not())
                .repeated()
                .collect::<String>()
                .spanned(),
        )
        .then(
            choice((just("\t"), just("  ")))
                .ignore_then(
                    any()
                        .to_slice()
                        .and_is(just("\n").not())
                        .repeated()
                        .collect::<String>()
                        .spanned(),
                )
                .repeated()
                .collect::<Vec<_>>(),
        )
        .map(|(first, mut remainder)| {
            remainder.insert(0, first);
            remainder
        })
        .validate(move |sm_str, map_extra, emitter| {
            let (result, errs) = composite_text(sm_str, strict_mark.clone());
            todo!()
        })
}