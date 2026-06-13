use chumsky::prelude::*;

use crate::data::{
    ParseData, TextVariants,
    error::{Block, Error, Expected},
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum FormulaExpected {
    Delimiter(usize),
    Text,
}

fn parser_formula_str<'src>(
    delimiter_count: usize,
) -> impl Parser<'src, &'src str, String, extra::Err<Error<'src>>> + Clone {
    let delimiter = "$".repeat(delimiter_count);
    choice((just("\\$").to("$"), just("\\\\").to("\\"), any().to_slice()))
        .and_is(just(delimiter.clone()).not())
        .labelled(Expected::Formula(FormulaExpected::Text))
        .repeated()
        .collect()
        .delimited_by(
            just(delimiter.clone()).labelled(Expected::Formula(FormulaExpected::Delimiter(
                delimiter_count,
            ))),
            just(delimiter).labelled(Expected::Formula(FormulaExpected::Delimiter(
                delimiter_count,
            ))),
        )
}

pub fn parser_inline_formula<'src>()
-> impl Parser<'src, &'src str, TextVariants, extra::Err<Error<'src>>> + Clone {
    parser_formula_str(1).map(TextVariants::InlineFormula)
}

pub fn parser_formula<'src>() -> impl Parser<'src, &'src str, ParseData, extra::Err<Error<'src>>> + Clone {
    parser_formula_str(2)
        .map(ParseData::Formula)
        .map_err(|err| err.set_target_block(Block::Formula))
}

#[cfg(test)]
mod tests {
    use chumsky::label::LabelError;
    pub use super::*;
    
    mod formula_str {
        use super::*;

        #[test]
        fn simple_1() {
            let input = "$ab\\$oba$";
            assert_eq!(
                parser_formula_str(1).parse(input).into_result(),
                Ok("ab$oba".to_string())
            );
        }

        #[test]
        fn simple_2() {
            let input = "$$ab\\$o$ba$$";
            assert_eq!(
                parser_formula_str(2).parse(input).into_result(),
                Ok("ab$o$ba".to_string())
            );
        }
    }

    #[test]
    fn simple_2() {
        let input = "$$ab\\$oba$$";
        assert_eq!(
            parser_formula().parse(input).into_result(),
            Ok(ParseData::Formula("ab$oba".to_string()))
        );
    }

    #[test]
    fn error() {
        let input = "$$ab\\$o$ba";
        assert_eq!(
            parser_formula().parse(input).into_result(),
            Err(vec![Error::expected_found(
                vec![Expected::Formula(FormulaExpected::Text), Expected::Formula(FormulaExpected::Delimiter(2))],
                None,
                (10..10).into()
            ).set_target_block(Block::Formula)])
        );
    }
}
