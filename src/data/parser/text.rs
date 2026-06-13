use chumsky::prelude::*;

use crate::data::{
    Text, TextVariants,
    error::{Error, Expected},
    parser::formula::parser_inline_formula,
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TextExpected {
    BoldDelimiter,
    ItalicDelimiter,
    UnderlinedDelimiter,
    StruckThroughDelimiter,
    UnbreakableTextDelimiter,
    LinkDelimiter,
    NameDelimiter,
    NameSeparator,
    UnexpectedSpecialCharacter,
    Screening,
    Text,
}

fn parser_base_text<'src>() -> impl Parser<'src, &'src str, String, extra::Err<Error<'src>>> {
    choice((
        just("\\*")
            .to("*")
            .labelled(Expected::Text(TextExpected::Screening)),
        just("\\_")
            .to("_")
            .labelled(Expected::Text(TextExpected::Screening)),
        just("\\~")
            .to("~")
            .labelled(Expected::Text(TextExpected::Screening)),
        just("\\`")
            .to("`")
            .labelled(Expected::Text(TextExpected::Screening)),
        just("\\@")
            .to("@")
            .labelled(Expected::Text(TextExpected::Screening)),
        just("\\$")
            .to("$")
            .labelled(Expected::Text(TextExpected::Screening)),
        just("\\)")
            .to(")")
            .labelled(Expected::Text(TextExpected::Screening)),
        just("\\\\")
            .to("\\")
            .labelled(Expected::Text(TextExpected::Screening)),
        any()
            .to_slice()
            .labelled(Expected::Text(TextExpected::Text)),
    ))
    .and_is(
        choice((
            just("*"),
            just("__"),
            just("~~"),
            just("`"),
            just("@"),
            just("$"),
            just(")"),
            just("\n"),
        ))
        .not(),
    )
    .labelled(Expected::Text(TextExpected::Text))
    .repeated()
    .at_least(1)
    .collect()
    .and_is(
        choice((
            just("#").repeated().at_least(1).then(just(" ")).ignored(),
            just("-").then(just(" ")).ignored(),
        ))
        .not(),
    )
}

pub fn parser_text<'src>() -> impl Parser<'src, &'src str, Text, extra::Err<Error<'src>>> {
    choice((
        parser_base_text()
            .filter(|a| !a.is_empty())
            .map(TextVariants::Text),
        parser_base_text()
            .delimited_by(
                just("**").labelled(Expected::Text(TextExpected::BoldDelimiter)),
                just("**").labelled(Expected::Text(TextExpected::BoldDelimiter)),
            )
            .map(TextVariants::Bold),
        parser_base_text()
            .delimited_by(
                just("*").labelled(Expected::Text(TextExpected::ItalicDelimiter)),
                just("*").labelled(Expected::Text(TextExpected::ItalicDelimiter)),
            )
            .map(TextVariants::Italic),
        parser_base_text()
            .delimited_by(
                just("__").labelled(Expected::Text(TextExpected::UnderlinedDelimiter)),
                just("__").labelled(Expected::Text(TextExpected::UnderlinedDelimiter)),
            )
            .map(TextVariants::Underlined),
        parser_base_text()
            .delimited_by(
                just("~~").labelled(Expected::Text(TextExpected::StruckThroughDelimiter)),
                just("~~").labelled(Expected::Text(TextExpected::StruckThroughDelimiter)),
            )
            .map(TextVariants::StruckThrough),
        parser_base_text()
            .delimited_by(
                just("`").labelled(Expected::Text(TextExpected::UnbreakableTextDelimiter)),
                just("`").labelled(Expected::Text(TextExpected::UnbreakableTextDelimiter)),
            )
            .map(TextVariants::UnbreakableText),
        parser_inline_formula(),
        just("@")
            .ignore_then(
                parser_base_text()
                    .delimited_by(
                        just("(").labelled(Expected::Text(TextExpected::NameDelimiter)),
                        just(")").labelled(Expected::Text(TextExpected::NameDelimiter)),
                    )
                    .separated_by(just(':').labelled(Expected::Text(TextExpected::NameSeparator)))
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .map(TextVariants::Link),
    ))
    .repeated()
    .at_least(1)
    .collect()
}

#[cfg(test)]
mod tests {
    pub use super::*;
    use crate::data::error::Expected;
    use chumsky::label::LabelError;

    mod base_text {
        use super::*;
        use crate::data::error::Expected;
        use chumsky::{label::LabelError, util::Maybe};

        #[test]
        fn simple() {
            let input = "asdfasdfasdfasdfasdfsdfcef";
            assert_eq!(
                parser_base_text().parse(input).into_result(),
                Ok(input.to_string())
            );
        }

        #[test]
        fn screening() {
            let input = "asdfasdf\\~asdf\\`asdfa\\*dfsd\\_fcef";
            assert_eq!(
                parser_base_text().parse(input).into_result(),
                Ok(input.to_string().replace("\\", ""))
            );
        }

        #[test]
        fn new_line() {
            let input = "aaa\nbbb";
            assert_eq!(
                parser_base_text().parse(input).into_result(),
                Err(vec![Error::expected_found(
                    vec![
                        Expected::Text(TextExpected::Text),
                        Expected::Other,
                        Expected::Text(TextExpected::Screening)
                    ],
                    Some(Maybe::Ref(&'\n')),
                    (3..4).into()
                )])
            );
        }

        #[test]
        fn error() {
            let input = "1234**sdfsdf";
            assert_eq!(
                parser_base_text().parse(input).into_result(),
                Err(vec![Error::expected_found(
                    vec![
                        Expected::Text(TextExpected::Text),
                        Expected::Other,
                        Expected::Text(TextExpected::Screening)
                    ],
                    Some(Maybe::Ref(&'*')),
                    (4..5).into()
                )])
            );
        }

        #[test]
        fn empty() {
            let input = "";
            assert_eq!(
                parser_base_text().parse(input).into_result(),
                Err(vec![Error::expected_found(
                    vec![
                        Expected::Text(TextExpected::Text),
                        Expected::Text(TextExpected::Screening)
                    ],
                    None,
                    (0..0).into()
                )])
            );
        }
    }

    #[test]
    fn base() {
        let test_str = "bib\\* bab **bub**__beb s sis\\___ff~~rr~~ `123 45` *\\** @(ss 1):(ss 1.1)";
        assert_eq!(
            parser_text().parse(test_str).into_result(),
            Ok(vec![
                TextVariants::Text("bib* bab ".to_string()),
                TextVariants::Bold("bub".to_string()),
                TextVariants::Underlined("beb s sis_".to_string()),
                TextVariants::Text("ff".to_string()),
                TextVariants::StruckThrough("rr".to_string()),
                TextVariants::Text(" ".to_string()),
                TextVariants::UnbreakableText("123 45".to_string()),
                TextVariants::Text(" ".to_string()),
                TextVariants::Italic("*".to_string()),
                TextVariants::Text(" ".to_string()),
                TextVariants::Link(vec!["ss 1".to_string(), "ss 1.1".to_string()])
            ])
        );
    }

    #[test]
    fn empty() {
        let input = "";
        assert_eq!(
            parser_base_text().parse(input).into_result(),
            Err(vec![Error::expected_found(
                vec![
                    Expected::Text(TextExpected::Text),
                    Expected::Text(TextExpected::Screening)
                ],
                None,
                (0..0).into()
            )])
        );
    }
}
