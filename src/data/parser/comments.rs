use crate::data::{PreParseData, error::Error};
use chumsky::prelude::*;

pub fn comments<'src>() -> impl Parser<'src, &'src str, PreParseData, extra::Err<Error>> + Clone {
    just("% ")
        .ignore_then(
            any()
                .to_slice()
                .and_is(just("\n").not())
                .repeated()
                .collect::<Vec<_>>()
                .map(|chars| chars.join("")),
        )
        .separated_by(just("\n"))
        .at_least(1)
        .collect()
        .map(|text: Vec<_>| PreParseData::Comments(text.join("\n")))
}

#[cfg(test)]
mod tests {
    use crate::data::error::Expected;

    use super::*;
    use crate::data::PreParseData;
    use chumsky::{label::LabelError, util::Maybe};

    #[test]
    fn base_test() {
        let input = "% ddddd";
        assert_eq!(
            comments().parse(input).into_result(),
            Ok(PreParseData::Comments("ddddd".to_string()))
        );

        let input = "% ddddd\n% ddddd";
        assert_eq!(
            comments().parse(input).into_result(),
            Ok(PreParseData::Comments("ddddd\nddddd".to_string()))
        );

        let input = "%ddddd\n%ddddd";
        assert_eq!(
            comments().parse(input).into_result(),
            Err(vec![Error::expected_found(
                vec![Expected::Other,],
                Some(Maybe::Ref(&'d')),
                (1..2).into()
            )])
        );
    }
}
