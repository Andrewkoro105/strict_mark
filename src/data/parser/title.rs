use crate::data::{PreParseData, Title, error::Error, parser::text::text};
use chumsky::{IterParser, Parser, extra, prelude::just};

pub fn title<'src>() -> impl Parser<'src, &'src str, PreParseData, extra::Err<Error>> + Clone {
    just('#')
        .repeated()
        .at_least(1)
        .count()
        .then_ignore(just(' '))
        .then(text().and_is(just("\n").not()))
        .then_ignore(just("\n").or_not())
        .map(|(level, text)| PreParseData::Title(Title { level, text }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{TextVariants, Title};

    #[test]
    fn base_test() {
        let input = "# ddddd";
        assert_eq!(
            title().parse(input).into_result(),
            Ok(PreParseData::Title(Title {
                level: 1,
                text: vec![TextVariants::Text("ddddd".to_string())]
            }))
        );

        let input = "## aaaaaa";
        assert_eq!(
            title().parse(input).into_result(),
            Ok(PreParseData::Title(Title {
                level: 2,
                text: vec![TextVariants::Text("aaaaaa".to_string())]
            }))
        );

        let input = "###################### cccccc";
        assert_eq!(
            title().parse(input).into_result(),
            Ok(PreParseData::Title(Title {
                level: 22,
                text: vec![TextVariants::Text("cccccc".to_string())]
            }))
        );
    }
}
