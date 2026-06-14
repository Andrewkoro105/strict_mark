use chumsky::{IterParser, Parser, extra, prelude::just};

use crate::data::{ParseData, error::Error, parser::text::text};

pub fn title<'src>() -> impl Parser<'src, &'src str, ParseData, extra::Err<Error<'src>>> + Clone {
    just('#')
        .repeated()
        .at_least(1)
        .count()
        .then_ignore(just(' '))
        .then(text().and_is(just("\n").not()))
        .then_ignore(just("\n").or_not())
        .map(|(level, text)| ParseData::Title { level, text })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::TextVariants;

    #[test]
    fn base_test() {
        let input = "# ddddd";
        assert_eq!(
            title().parse(input).into_result(),
            Ok(ParseData::Title {
                level: 1,
                text: vec![TextVariants::Text("ddddd".to_string())]
            })
        );

        let input = "## aaaaaa";
        assert_eq!(
            title().parse(input).into_result(),
            Ok(ParseData::Title {
                level: 2,
                text: vec![TextVariants::Text("aaaaaa".to_string())]
            })
        );

        let input = "###################### cccccc";
        assert_eq!(
            title().parse(input).into_result(),
            Ok(ParseData::Title {
                level: 22,
                text: vec![TextVariants::Text("cccccc".to_string())]
            })
        );
    }
}
