use std::collections::HashMap;

use chumsky::{IterParser, Parser, extra, prelude::*, span::Spanned, text};

use crate::data::{
    ParamData, ParamType, ParamValues, Params,
    error::{Error, Expected},
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ParamsExpected {
    UnknownName {
        name: String,
        known_name: Vec<String>,
    },
    UnknownValue(Vec<String>),
    IncorrectType(ParamType),
    Separator,
    Delimited,
    Name,
    AnyValue,
    List,
    ListDelimited,
    ListSeparator,
    Bool,
    I32,
    F32,
}

pub fn params<'src>() -> impl Parser<'src, &'src str, Params, extra::Err<Error>> + Clone {
    text::ident()
        .labelled(Expected::Params(ParamsExpected::Name))
        .spanned()
        .padded()
        .then(
            just('=')
                .ignore_then(recursive(|value| {
                    choice((
                        just("true")
                            .to(true)
                            .or(just("false").to(false))
                            .labelled(Expected::Params(ParamsExpected::Bool))
                            .map(ParamValues::Bool),
                        just("-")
                            .to(-1.)
                            .or_not()
                            .then(
                                text::digits(10)
                                    .to_slice()
                                    .then(just("."))
                                    .then(text::digits(10).to_slice())
                                    .map(|((int, dot), decimal_fraction)| {
                                        [int, dot, decimal_fraction].concat()
                                    }),
                            )
                            .then(just("%").to(100.).or_not())
                            .labelled(Expected::Params(ParamsExpected::F32))
                            .map(|((dec, digit), percent)| {
                                ParamValues::F32(
                                    (digit.parse::<f32>().unwrap() * dec.unwrap_or(1.))
                                        / percent.unwrap_or(1.),
                                )
                            }),
                        just("-")
                            .to(-1)
                            .or_not()
                            .then(text::digits(10).to_slice())
                            .then(just("%").to(100.).or_not())
                            .labelled(Expected::Params(ParamsExpected::I32))
                            .map(
                                |((dec, digit), percent): ((Option<_>, &str), _)| match percent {
                                    Some(percent) => ParamValues::F32(
                                        (digit.parse::<f32>().unwrap() * dec.unwrap_or(1) as f32)
                                            / percent,
                                    ),
                                    None => ParamValues::I32(
                                        digit.parse::<i32>().unwrap() * dec.unwrap_or(1),
                                    ),
                                },
                            ),
                        value
                            .labelled(Expected::Params(ParamsExpected::List))
                            .map(|value: Spanned<ParamValues>| (value.inner, value.span))
                            .separated_by(
                                just(",").labelled(Expected::Params(ParamsExpected::ListSeparator)),
                            )
                            .collect()
                            .delimited_by(
                                just("[").labelled(Expected::Params(ParamsExpected::ListDelimited)),
                                just("]").labelled(Expected::Params(ParamsExpected::ListDelimited)),
                            )
                            .map(ParamValues::List),
                        text::ident()
                            .to_slice()
                            .labelled(Expected::Params(ParamsExpected::AnyValue))
                            .map(|ident: &str| ParamValues::Value(ident.to_string())),
                    ))
                    .spanned()
                    .padded()
                }))
                .or_not(),
        )
        .map(|(name, value): (Spanned<&'src str>, _)| {
            (
                name.inner.to_string(),
                ParamData {
                    value: value
                        .clone()
                        .map(|spanned| spanned.inner)
                        .unwrap_or(ParamValues::Bool(true)),
                    name_span: name.span,
                    value_span: value.map(|spanned| spanned.span),
                },
            )
        })
        .separated_by(just(',').labelled(Expected::Params(ParamsExpected::Separator)))
        .collect::<Params>()
        .delimited_by(
            just('{').labelled(Expected::Params(ParamsExpected::Delimited)),
            just('}').labelled(Expected::Params(ParamsExpected::Delimited)),
        )
}

pub fn unknown_variables<'src>(
    variables: HashMap<String, ParamData>,
    known_name: Vec<String>,
) -> Vec<Error> {
    variables
        .iter()
        .map(|(name, data)| {
            Error::new(
                vec![Expected::Params(ParamsExpected::UnknownName {
                    name: name.clone(),
                    known_name: known_name.clone(),
                })],
                None,
                data.name_span,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::{label::LabelError, span::SimpleSpan, util::Maybe};
    use sugar::hashmap;

    #[test]
    fn empty() {
        let input = "{}";
        assert_eq!(params().parse(input).into_result(), Ok(hashmap! {}));
    }

    #[test]
    fn default_true() {
        let input = "{a}";
        assert_eq!(
            params().parse(input).into_result(),
            Ok(hashmap! {
                "a".to_string() => ParamData{
                    value: ParamValues::Bool(true),
                    name_span: SimpleSpan {
                        start: 1,
                        end: 2,
                        ..Default::default()
                    },
                    value_span: None,
                },
            })
        );
    }

    #[test]
    fn bool() {
        let input = "{a = true, b = false}";
        assert_eq!(
            params().parse(input).into_result(),
            Ok(hashmap! {
                "a".to_string() => ParamData{
                    value: ParamValues::Bool(true),
                    name_span: (1..2).into(),
                    value_span: Some((5..9).into()),
                },
                "b".to_string() => ParamData{
                    value: ParamValues::Bool(false),
                    name_span: (11..12).into(),
                    value_span: Some((15..20).into()),
                },
            })
        );
    }

    #[test]
    fn int() {
        let input = "{a = 34, b = 42}";
        assert_eq!(
            params().parse(input).into_result(),
            Ok(hashmap! {
                "a".to_string() => ParamData{
                    value: ParamValues::I32(34),
                    name_span: (1..2).into(),
                    value_span: Some((5..7).into()),
                },
                "b".to_string() => ParamData{
                    value: ParamValues::I32(42),
                    name_span: (9..10).into(),
                    value_span: Some((13..15).into()),
                },
            })
        );
    }

    #[test]
    fn float() {
        let input = "{a = 34.42, b = -42.32, c = 69%}";
        assert_eq!(
            params().parse(input).into_result(),
            Ok(hashmap! {
                "a".to_string() => ParamData{
                    value: ParamValues::F32(34.42),
                    name_span: (1..2).into(),
                    value_span: Some((5..10).into()),
                },
                "b".to_string() => ParamData{
                    value: ParamValues::F32(-42.32),
                    name_span: (12..13).into(),
                    value_span: Some((16..22).into()),
                },
                "c".to_string() => ParamData{
                    value: ParamValues::F32(0.69),
                    name_span: (24..25).into(),
                    value_span: Some((28..31).into()),
                },
            })
        );
    }

    #[test]
    fn indent() {
        let input = "{a = aaa, b = b22}";
        assert_eq!(
            params().parse(input).into_result(),
            Ok(hashmap! {
                "a".to_string() => ParamData{
                    value: ParamValues::Value("aaa".into()),
                    name_span: (1..2).into(),
                    value_span: Some((5..8).into()),
                },
                "b".to_string() => ParamData{
                    value: ParamValues::Value("b22".into()),
                    name_span: (10..11).into(),
                    value_span: Some((14..17).into()),
                },
            })
        );
    }

    #[test]
    fn list() {
        let input = "{a = [], b = [-1, 0, 2.3, aaa, [-1, 0, 2.3, aaa]]}";
        assert_eq!(
            params().parse(input).into_result(),
            Ok(hashmap! {
                "a".to_string() => ParamData{
                    value: ParamValues::List(vec![]),
                    name_span: (1..2).into(),
                    value_span: Some((5..7).into()),
                },
                "b".to_string() => ParamData{
                    value: ParamValues::List(vec![
                        (ParamValues::I32(-1), (14..16).into()),
                        (ParamValues::I32(0), (18..19).into()),
                        (ParamValues::F32(2.3), (21..24).into()),
                        (ParamValues::Value("aaa".into()), (26..29).into()),
                        (ParamValues::List(vec![
                            (ParamValues::I32(-1), (32..34).into()),
                            (ParamValues::I32(0), (36..37).into()),
                            (ParamValues::F32(2.3), (39..42).into()),
                            (ParamValues::Value("aaa".into()), (44..47).into()),
                        ]), (31..48).into())
                    ]),
                    name_span: (9..10).into(),
                    value_span: Some((13..49).into()),
                },
            })
        );
    }

    #[test]
    fn error() {
        let input = "{a";
        assert_eq!(
            params().parse(input).into_result(),
            Err(vec![Error::expected_found(
                vec![
                    Expected::Other,
                    Expected::Params(ParamsExpected::Delimited),
                    Expected::Params(ParamsExpected::Separator),
                ],
                None,
                (2..2).into()
            )])
        );

        let input = "{a b}";
        assert_eq!(
            params().parse(input).into_result(),
            Err(vec![Error::expected_found(
                vec![
                    Expected::Other,
                    Expected::Params(ParamsExpected::Delimited),
                    Expected::Params(ParamsExpected::Separator),
                ],
                Some(Maybe::Ref(&'b')),
                (3..4).into()
            )])
        );
    }

    #[test]
    fn list_error() {
        let input = "{a = [a}";
        assert_eq!(
            params().parse(input).into_result(),
            Err(vec![Error::expected_found(
                vec![
                    Expected::Other,
                    Expected::Params(ParamsExpected::ListSeparator),
                    Expected::Params(ParamsExpected::ListDelimited),
                ],
                Some(Maybe::Ref(&'}')),
                (7..8).into()
            )])
        );

        let input = "{a = [a b]}";
        assert_eq!(
            params().parse(input).into_result(),
            Err(vec![Error::expected_found(
                vec![
                    Expected::Params(ParamsExpected::ListSeparator),
                    Expected::Params(ParamsExpected::ListDelimited),
                ],
                Some(Maybe::Ref(&'b')),
                (8..9).into()
            )])
        );
    }
}
