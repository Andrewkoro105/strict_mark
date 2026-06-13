use chumsky::prelude::*;

use crate::data::{
    ParagraphType, ParamType, ParamValues, ParsData, Text, TextVariants,
    error::{Block, Error, Expected},
    parser::{
        params::{ParamsExpected, parser_params, unknown_variables},
        text::parser_text,
    },
};

fn parser_base_paragraph<'src>() -> impl Parser<'src, &'src str, Text, extra::Err<Error<'src>>> {
    parser_text()
        .then(
            choice((
                just("\n\n").to(TextVariants::Text("\n".into())),
                just("\n").to(TextVariants::PhantomNewLine),
            ))
            .or_not(),
        )
        .map(|(mut text, new_line)| {
            if let Some(new_line) = new_line {
                text.push(new_line);
            }
            text
        })
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|texts| texts.into_iter().flatten().collect())
}

pub fn parser_paragraph<'src>() -> impl Parser<'src, &'src str, ParsData, extra::Err<Error<'src>>> {
    parser_params()
        .or_not()
        .map(Option::unwrap_or_default)
        .then(parser_base_paragraph())
        .map_err(|err| err.set_target_block(Block::Paragraph))
        .validate(|(mut params, text), map_extra, emitter| {
            let paragraph_type = params
                .remove(&"type".to_string())
                .map(|paragraph_type_data| match paragraph_type_data.value {
                    ParamValues::Value(paragraph_type_str) => {
                        Some(match paragraph_type_str.as_str() {
                            "text" => ParagraphType::Text,
                            "quote" => ParagraphType::Quote,
                            "footnote" => ParagraphType::Footnote,
                            _ => ParagraphType::Other(paragraph_type_str),
                        })
                    }
                    _ => {
                        emitter.emit(Error::new(
                            vec![Expected::Params(ParamsExpected::IncorrectType(
                                ParamType::Value,
                            ))],
                            None,
                            map_extra.span(),
                        ));
                        None
                    }
                })
                .flatten()
                .unwrap_or(ParagraphType::default());

            unknown_variables(params, vec!["type".into()])
                .into_iter()
                .for_each(|err| emitter.emit(err));

            ParsData::Paragraph {
                paragraph_type,
                text,
            }
        })
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn one_str() {
        let test_str = "bib\\* bab **bub**__beb s sis\\___ff~~rr~~ `123 45` *\\** @(ss 1):(ss 1.1)";
        assert_eq!(
            parser_paragraph().parse(test_str).into_result(),
            Ok(ParsData::Paragraph {
                paragraph_type: ParagraphType::Text,
                text: vec![
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
                ]
            })
        );
    }

    #[test]
    fn phantom_new_line() {
        let test_str = "bub bab \n bib beb";
        assert_eq!(
            parser_paragraph().parse(test_str).into_result(),
            Ok(ParsData::Paragraph {
                paragraph_type: ParagraphType::Text,
                text: vec![
                    TextVariants::Text("bub bab ".to_string()),
                    TextVariants::PhantomNewLine,
                    TextVariants::Text(" bib beb".to_string()),
                ]
            })
        );
    }

    #[test]
    fn new_line() {
        let test_str = "bub bab \n\n bib beb";
        assert_eq!(
            parser_paragraph().parse(test_str).into_result(),
            Ok(ParsData::Paragraph {
                paragraph_type: ParagraphType::Text,
                text: vec![
                    TextVariants::Text("bub bab ".to_string()),
                    TextVariants::Text("\n".to_string()),
                    TextVariants::Text(" bib beb".to_string()),
                ]
            })
        );
    }

    #[test]
    fn not_default_type() {
        let test_str = "{type = quote}bub bab \n\n bib beb";
        assert_eq!(
            parser_paragraph().parse(test_str).into_result(),
            Ok(ParsData::Paragraph {
                paragraph_type: ParagraphType::Quote,
                text: vec![
                    TextVariants::Text("bub bab ".to_string()),
                    TextVariants::Text("\n".to_string()),
                    TextVariants::Text(" bib beb".to_string()),
                ]
            })
        );

        let test_str = "{type = fuf}bub bab \n\n bib beb";
        assert_eq!(
            parser_paragraph().parse(test_str).into_result(),
            Ok(ParsData::Paragraph {
                paragraph_type: ParagraphType::Other("fuf".into()),
                text: vec![
                    TextVariants::Text("bub bab ".to_string()),
                    TextVariants::Text("\n".to_string()),
                    TextVariants::Text(" bib beb".to_string()),
                ]
            })
        );
    }
}
