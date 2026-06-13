use chumsky::{IterParser, Parser, extra, prelude::just};

use crate::data::{ParsData, error::Error, parser::text::parser_text};

pub fn parser_title<'src>() -> impl Parser<'src, &'src str, ParsData, extra::Err<Error<'src>>> {
    just('#')
        .repeated()
        .at_least(1)
        .count()
        .then_ignore(just(' '))
        .then(parser_text().and_is(just("\n").not()))
        .map(|(level, text)| ParsData::Title { level, text })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::TextVariants;

    #[test]
    fn base_test() {
        let input = "# ddddd";
        assert_eq!(
            parser_title().parse(input).into_result(),
            Ok(ParsData::Title {
                level: 1,
                text: vec![TextVariants::Text("ddddd".to_string())]
            })
        );

        let input = "## aaaaaa";
        assert_eq!(
            parser_title().parse(input).into_result(),
            Ok(ParsData::Title {
                level: 2,
                text: vec![TextVariants::Text("aaaaaa".to_string())]
            })
        );

        let input = "###################### cccccc";
        assert_eq!(
            parser_title().parse(input).into_result(),
            Ok(ParsData::Title {
                level: 22,
                text: vec![TextVariants::Text("cccccc".to_string())]
            })
        );
    }
}
