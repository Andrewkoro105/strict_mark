use chumsky::prelude::*;

use crate::data::{
    EnumerateType, ParamType, ParamValues, PreParseData,
    error::{Block, BlockConvertor, Error, ErrorEditor, Expected},
    parser::params::{ParamsExpected, params, unknown_variables},
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EnumerateExpected {
    Marker,
    Text,
}

pub fn enumerate<'src>() -> impl Parser<'src, &'src str, PreParseData, extra::Err<Error>> + Clone {
    fn text<'src>(
        tab: impl Parser<'src, &'src str, (), extra::Err<Error>> + Clone,
    ) -> impl Parser<'src, &'src str, (Spanned<String>, Vec<Spanned<String>>), extra::Err<Error>> + Clone
    {
        any()
            .to_slice()
            .and_is(just("\n").not())
            .repeated()
            .collect::<String>()
            .spanned()
            .then_ignore(choice((just("\n").ignored(), end())))
            .labelled(Expected::Enumerate(EnumerateExpected::Text))
            .then(
                tab.ignore_then(
                    any()
                        .to_slice()
                        .and_is(just("\n").not())
                        .repeated()
                        .collect::<String>()
                        .spanned()
                        .then_ignore(choice((just("\n").ignored(), end()))),
                )
                .and_is(choice((just("}").ignored(), end())).not())
                .repeated()
                .collect::<Vec<Spanned<String>>>()
                .labelled(Expected::Enumerate(EnumerateExpected::Text)),
            )
    }

    params()
        .then_ignore(just("\n"))
        .or_not()
        .map(Option::unwrap_or_default)
        .then(
            just("- ")
                .labelled(Expected::Enumerate(EnumerateExpected::Marker))
                .ignore_then(choice((
                    text(choice((just("\t"), just("  "), just(""))).ignored())
                        .delimited_by(just("{"), just("}"))
                        .then_ignore(just("\n").or_not()),
                    text(choice((just("\t"), just("  "))).ignored()),
                )))
                .map(|(first, mut remainder)| {
                    remainder.insert(0, first);
                    let (strs, spans) = remainder
                        .into_iter()
                        .map(|remainder| (remainder.inner, remainder.span))
                        .unzip::<_, _, Vec<_>, Vec<_>>();
                    PreParseData::Pre {
                        data_str: strs.join("\n"),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans,
                        },
                    }
                })
                .repeated()
                .at_least(1)
                .collect::<Vec<PreParseData>>(),
        )
        .validate(|(mut params, data), map_extra, emitter| {
            let enumerate_type = params
                .remove(&"type".to_string())
                .map(|paragraph_type_data| match paragraph_type_data.value {
                    ParamValues::Value(paragraph_type_str) => {
                        Some(match paragraph_type_str.as_str() {
                            "default" => EnumerateType::Default,
                            "number" => EnumerateType::Number,
                            "mark" => EnumerateType::Mark,
                            "char" => EnumerateType::Char,
                            "bibliography" => EnumerateType::Bibliography,
                            "definitions" => EnumerateType::Definitions,
                            _ => EnumerateType::Other(paragraph_type_str),
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
                .unwrap_or(EnumerateType::default());

            unknown_variables(params, vec!["type".into()])
                .into_iter()
                .for_each(|err| emitter.emit(err));

            PreParseData::Enumerate {
                enumerate_type,
                data,
            }
        })
        .map_err(|err| err.set_target_block(Block::Enumerate(None)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_line() {
        let input = r#"- aboba1
- aboba2
- aboba3"#;

        assert_eq!(
            enumerate().parse(input).into_result(),
            Ok(PreParseData::Enumerate {
                enumerate_type: EnumerateType::Default,
                data: vec![
                    PreParseData::Pre {
                        data_str: "aboba1".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![SimpleSpan::from(2..8)]
                        },
                    },
                    PreParseData::Pre {
                        data_str: "aboba2".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![SimpleSpan::from(11..17)]
                        },
                    },
                    PreParseData::Pre {
                        data_str: "aboba3".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![SimpleSpan::from(20..26)]
                        },
                    }
                ]
            })
        );
    }

    #[test]
    fn parm() {
        let input1 = r#"{type = mark}
- aboba1
- aboba2
- aboba3"#;

        assert_eq!(
            enumerate().parse(input1).into_result(),
            Ok(PreParseData::Enumerate {
                enumerate_type: EnumerateType::Mark,
                data: vec![
                    PreParseData::Pre {
                        data_str: "aboba1".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![SimpleSpan::from(16..22)]
                        },
                    },
                    PreParseData::Pre {
                        data_str: "aboba2".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![SimpleSpan::from(25..31)]
                        },
                    },
                    PreParseData::Pre {
                        data_str: "aboba3".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![SimpleSpan::from(34..40)]
                        },
                    }
                ]
            })
        );

        let input2 = r#"{type = gebe}
- aboba1
- aboba2
- aboba3"#;

        assert_eq!(
            enumerate().parse(input2).into_result(),
            Ok(PreParseData::Enumerate {
                enumerate_type: EnumerateType::Other("gebe".into()),
                data: vec![
                    PreParseData::Pre {
                        data_str: "aboba1".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![SimpleSpan::from(16..22)]
                        },
                    },
                    PreParseData::Pre {
                        data_str: "aboba2".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![SimpleSpan::from(25..31)]
                        },
                    },
                    PreParseData::Pre {
                        data_str: "aboba3".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![SimpleSpan::from(34..40)]
                        },
                    }
                ]
            })
        );
    }

    #[test]
    fn multi_line() {
        let input = r#"- aboba1
  aboba1.1
  aboba1.2
  aboba1.3
- aboba2
- aboba3
  
  aboba3.1"#;

        assert_eq!(
            enumerate().parse(input).into_result(),
            Ok(PreParseData::Enumerate {
                enumerate_type: EnumerateType::Default,
                data: vec![
                    PreParseData::Pre {
                        data_str: "aboba1\naboba1.1\naboba1.2\naboba1.3".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![
                                SimpleSpan::from(2..8),
                                SimpleSpan::from(11..19),
                                SimpleSpan::from(22..30),
                                SimpleSpan::from(33..41),
                            ]
                        },
                    },
                    PreParseData::Pre {
                        data_str: "aboba2".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![SimpleSpan::from(44..50)]
                        },
                    },
                    PreParseData::Pre {
                        data_str: "aboba3\n\naboba3.1".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![
                                SimpleSpan::from(53..59),
                                SimpleSpan::from(62..62),
                                SimpleSpan::from(65..73)
                            ]
                        },
                    }
                ]
            })
        );
    }

    #[test]
    fn curly_brackets() {
        let input = r#"- {aboba1
  aboba1.1
  aboba1.2
  aboba1.3
}
- aboba2
- {aboba3

  aboba3.1
}"#;

        assert_eq!(
            enumerate().parse(input).into_result(),
            Ok(PreParseData::Enumerate {
                enumerate_type: EnumerateType::Default,
                data: vec![
                    PreParseData::Pre {
                        data_str: "aboba1\naboba1.1\naboba1.2\naboba1.3".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![
                                SimpleSpan::from(3..9),
                                SimpleSpan::from(12..20),
                                SimpleSpan::from(23..31),
                                SimpleSpan::from(34..42),
                            ]
                        },
                    },
                    PreParseData::Pre {
                        data_str: "aboba2".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![SimpleSpan::from(47..53)]
                        },
                    },
                    PreParseData::Pre {
                        data_str: "aboba3\n\naboba3.1".into(),
                        block_editor: ErrorEditor {
                            block_convertor: Some(BlockConvertor::Enumerate),
                            spans: vec![
                                SimpleSpan::from(57..63),
                                SimpleSpan::from(64..64),
                                SimpleSpan::from(67..75)
                            ]
                        },
                    }
                ]
            })
        );
    }
}
