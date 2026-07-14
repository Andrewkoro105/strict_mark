use crate::data::parser::{
    enumerate::EnumerateExpected, formula::FormulaExpected, params::ParamsExpected,
    text::TextExpected,
};
use chumsky::{
    DefaultExpected, error::Error as ChumskyError, label::LabelError, span::SimpleSpan,
    text::TextExpected as ChumskyTextExpected, util::MaybeRef,
};
use serde::{Deserialize, Serialize};
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum BlockConvertor {
    Enumerate,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Expected {
    Enumerate(EnumerateExpected),
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
    pub spans: Vec<SimpleSpan>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct ErrorEditor {
    pub block_convertor: Option<BlockConvertor>,
    pub spans: Vec<SimpleSpan>,
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
            spans: vec![span],
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
            spans: vec![span],
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
            spans: vec![span],
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
            spans: vec![span],
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

#[derive(PartialEq, Debug)]
struct NewPos {
    pos: usize,
    span: SimpleSpan,
}

impl ErrorEditor {
    fn pos_convertor(&self, pos: usize) -> Option<NewPos> {
        let mut len = 0;
        self.spans
            .iter()
            .find(|span| {
                let current_len = span.end - span.start;
                let result = len <= pos && pos < (len + current_len);
                if !result {
                    len += current_len;
                }
                result
            })
            .cloned()
            .map(|span| NewPos {
                pos: span.start + (pos - len),
                span,
            })
    }

    pub fn edit(&self, err: &mut Error) {
        if let Some(block_convertor) = self.block_convertor {
            err.expected.iter_mut().for_each(|(block, _)| {
                *block = match block_convertor {
                    BlockConvertor::Enumerate => Block::Enumerate(Some(Box::new(block.clone()))),
                }
            });
        }

        let err_clone = err.clone();
        if !self.spans.is_empty() {
            err.spans = err.spans.clone().into_iter().flat_map(|span| {
                let new_start =  self
                .pos_convertor(span.start)
                .expect(&format!(
                    "The error `{:?}`\ncannot be converted using `{:?}`\nbecause position {} is not within its span.", 
                    err_clone,
                    self,
                    span.start,
                ));
                let new_end = self
                    .pos_convertor(span.end)
                    .expect(&format!(
                        "The error `{:?}`\ncannot be converted using `{:?}`\nbecause position {} is not within its span.", 
                        err_clone,
                        self,
                        span.end
                    ));

                if new_start.span == new_end.span {
                    vec![SimpleSpan::from(new_start.pos..new_end.pos)]
                } else {
                    vec![
                        SimpleSpan::from(new_start.pos..new_start.span.end),
                        SimpleSpan::from(new_end.pos..new_end.span.start),
                    ]
                }
            }).collect();
        }
    }

    pub fn edit_errs(&self, err: &mut Vec<Error>) {
        err.iter_mut().for_each(|err| self.edit(err));
    }

    pub fn none() -> Self {
        Self {
            block_convertor: None,
            spans: vec![],
        }
    }
}

#[test]
fn test() {
    let error_editor = ErrorEditor {
        block_convertor: None,
        spans: vec![
            SimpleSpan::from(3..7),
            SimpleSpan::from(8..10),
            SimpleSpan::from(12..14),
        ],
    };

    assert_eq!(
        error_editor.pos_convertor(2),
        Some(NewPos {
            pos: 5,
            span: SimpleSpan::from(3..7)
        })
    );
    assert_eq!(
        error_editor.pos_convertor(7),
        Some(NewPos {
            pos: 13,
            span: SimpleSpan::from(12..14)
        })
    );
}
