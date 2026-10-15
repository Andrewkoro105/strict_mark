use crate::data::{
    PreParseData, ResourceInsertion, error::Error, parser::paragraph::base_paragraph,
};
use chumsky::{
    IterParser, Parser, extra,
    prelude::just,
    primitive::{any, choice},
};

pub fn resource_insertion<'src>()
-> impl Parser<'src, &'src str, PreParseData, extra::Err<Error>> + Clone {
    just('#')
        .ignore_then(
            any()
                .and_is(choice((just(")"), just("\n"))).not())
                .repeated()
                .collect::<String>()
                .delimited_by(just("("), just(")")),
        )
        .then(base_paragraph().delimited_by(
            just("{").then(just("\n").or_not()),
            just("}"),
        ))
        .map(|(path, caption)| {
            PreParseData::ResourceInsertion(ResourceInsertion {
                path: path.into(),
                caption,
            })
        })
}

#[cfg(test)]
mod test {
    use crate::data::TextVariants;
use crate::data::parser::insertion::resource_insertion;
    use crate::data::{
        PreParseData, ResourceInsertion
    };
    use chumsky::Parser;

    #[test]
    fn max() {
        let test_str = r#"#(path.png){
aboba __aboba__ aboba
}"#;
        assert_eq!(
            resource_insertion().parse(test_str).into_result(),
            Ok(PreParseData::ResourceInsertion(ResourceInsertion {
                path: "path.png".into(),
                caption: vec![
                    TextVariants::Text("aboba ".to_owned()),
                    TextVariants::Underlined("aboba".to_owned()),
                    TextVariants::Text(" aboba".to_owned()),
                    TextVariants::PhantomNewLine
                ],
            }))
        );
    }

    #[test]
    fn min() {
        let test_str = r#"#(path.png){aboba __aboba__ aboba}"#;
        assert_eq!(
            resource_insertion().parse(test_str).into_result(),
            Ok(PreParseData::ResourceInsertion(ResourceInsertion {
                path: "path.png".into(),
                caption: vec![
                    TextVariants::Text("aboba ".to_owned()),
                    TextVariants::Underlined("aboba".to_owned()),
                    TextVariants::Text(" aboba".to_owned()),
                ],
            }))
        );
    }
}
