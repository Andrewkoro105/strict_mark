use chumsky::prelude::*;
use crate::data::ParsData;

pub fn parser_comments<'src>() -> impl Parser<'src, &'src str, ParsData> {
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
        .map(|text: Vec<_>| ParsData::Comments(text.join("\n")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chumsky::error::EmptyErr;

    #[test]
    fn base_test() {
        let input = "% ddddd";
        assert_eq!(
            parser_comments().parse(input).into_result(),
            Ok(ParsData::Comments("ddddd".to_string()))
        );

        let input = "% ddddd\n% ddddd";
        assert_eq!(
            parser_comments().parse(input).into_result(),
            Ok(ParsData::Comments("ddddd\nddddd".to_string()))
        );

        let input = "%ddddd\n%ddddd";
        assert_eq!(
            parser_comments().parse(input).into_result(),
            Err(vec![EmptyErr::default()])
        );
    }
}
