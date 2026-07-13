use crate::data::parser::{formula::FormulaExpected, params::ParamsExpected, text::TextExpected};
use chumsky::{
    DefaultExpected, error::Error as ChumskyError, label::LabelError, span::SimpleSpan,
    text::TextExpected as ChumskyTextExpected, util::MaybeRef,
};
use std::collections::HashSet;
#[cfg(test)]
use std::hash::{BuildHasherDefault, DefaultHasher};

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub enum Block {
    #[default]
    Unknown,
    Paragraph,
    Formula,
    Enumerate(Option<Box<Self>>),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Expected {
    Params(ParamsExpected),
    Formula(FormulaExpected),
    Text(TextExpected),
    TabSpace,
    Space,
    Other,
}

#[derive(Clone, PartialEq, Debug)]
pub struct NotEnd<T>(pub Option<T>);

#[derive(Clone, PartialEq, Debug)]
pub struct Error {
    expected: Vec<(Block, Vec<Expected>)>,
    found: Option<NotEnd<char>>,
    pub span: SimpleSpan,
}

impl<'src> LabelError<'src, &'src str, DefaultExpected<'src, char>> for Error {
    fn expected_found<E: IntoIterator<Item = DefaultExpected<'src, char>>>(
        expected: E,
        found: Option<MaybeRef<'src, char>>,
        span: SimpleSpan,
    ) -> Self {
        Self {
            expected: vec![(
                Block::default(),
                expected.into_iter().map(|_| Expected::Other).collect(),
            )],
            found: Some(NotEnd(found.map(|ch| match ch {
                chumsky::util::Maybe::Ref(ch) => *ch,
                chumsky::util::Maybe::Val(ch) => ch,
            }))),
            span,
        }
    }
}

impl<'src> LabelError<'src, &'src str, ChumskyTextExpected<()>> for Error {
    fn expected_found<E: IntoIterator<Item = ChumskyTextExpected<()>>>(
        expected: E,
        found: Option<MaybeRef<'src, char>>,
        span: SimpleSpan,
    ) -> Self {
        Self {
            expected: vec![(
                Block::default(),
                expected.into_iter().map(|_| Expected::Other).collect(),
            )],
            found: Some(NotEnd(found.map(|ch| match ch {
                chumsky::util::Maybe::Ref(ch) => *ch,
                chumsky::util::Maybe::Val(ch) => ch,
            }))),
            span,
        }
    }
}

impl<'src> LabelError<'src, &'src str, Expected> for Error {
    fn expected_found<E: IntoIterator<Item = Expected>>(
        expected: E,
        found: Option<MaybeRef<'src, char>>,
        span: SimpleSpan,
    ) -> Self {
        Self {
            expected: vec![(Block::default(), expected.into_iter().collect())],
            found: Some(NotEnd(found.map(|ch| match ch {
                chumsky::util::Maybe::Ref(ch) => *ch,
                chumsky::util::Maybe::Val(ch) => ch,
            }))),
            span,
        }
    }

    fn label_with(&mut self, label: Expected) {
        for (_, expected) in self.expected.iter_mut() {
            if expected.is_empty() {
                *expected = vec![label.clone()];
            } else {
                for expected in expected.iter_mut() {
                    if *expected == Expected::Other {
                        *expected = label.clone();
                    }
                }
            }
        }
    }
}

impl<'src> ChumskyError<'src, &'src str> for Error {
    fn merge(mut self, other: Self) -> Self {
        self.expected.extend(other.expected);

        #[cfg(test)]
        let mut unknown_block_expected =
            HashSet::<Expected, BuildHasherDefault<DefaultHasher>>::default();
        #[cfg(not(test))]
        let mut unknown_block_expected = HashSet::new();

        self.expected = self
            .expected
            .into_iter()
            .filter(|(block, expected)| {
                if *block == Block::Unknown {
                    unknown_block_expected.extend(expected.clone());
                    false
                } else {
                    true
                }
            })
            .collect();

        self.expected.push((
            Block::Unknown,
            unknown_block_expected.into_iter().collect::<Vec<_>>(),
        ));

        self
    }
}

impl<'src> Error {
    pub fn new(expected: Vec<Expected>, found: Option<NotEnd<char>>, span: SimpleSpan) -> Self {
        Self {
            expected: vec![(Block::Unknown, expected)],
            found,
            span,
        }
    }

    pub fn set_target_block(mut self, target_block: Block) -> Self {
        self.expected.iter_mut().for_each(|(block, _)| {
            if *block == Block::Unknown {
                *block = target_block.clone();
            }
        });
        self
    }

    pub fn map_block(mut self, f: impl Fn(Block) -> Block) -> Self {
        self.expected.iter_mut().for_each(|(block, _)| {
            *block = f(block.clone());
        });
        self
    }
}
