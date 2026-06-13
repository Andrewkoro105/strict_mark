pub mod error;
pub mod parser;

use std::{collections::HashMap, path::PathBuf};

use chumsky::span::SimpleSpan;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ParagraphType {
    #[default]
    Text,
    Footnote,
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum EnumerateType {
    Number,
    Mark,
    Char,
    Bibliography,
    Definitions,
    Other(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextVariants {
    PhantomNewLine,
    Text(String),
    Bold(String),
    Italic(String),
    Underlined(String),
    StruckThrough(String),
    UnbreakableText(String),
    Link(Vec<String>),
    InlineFormula(String),
}

pub type Text = Vec<TextVariants>;

#[derive(Debug, Clone, PartialEq)]
pub enum ParamValues {
    List(Vec<(Self, SimpleSpan)>),
    Bool(bool),
    I32(i32),
    F32(f32),
    Value(String),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ParamType {
    List,
    Bool,
    I32,
    F32,
    Value,
}

#[derive(PartialEq, Debug)]
pub struct ParamData {
    pub value: ParamValues,
    pub name_span: SimpleSpan,
    pub value_span: Option<SimpleSpan>,
}

pub type Params = HashMap<String, ParamData>;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseData {
    List(Vec<Self>),
    Name {
        name: String,
        data: Box<ParseData>,
    },
    Comments(String),
    Title {
        level: usize,
        text: Text,
    },
    Paragraph {
        paragraph_type: ParagraphType,
        text: Text,
    },
    Formula(String),
    InsertPage(PathBuf),
    InsertContent {
        path: PathBuf,
        caption: Text,
    },
    Enumerate {
        enumerate_type: EnumerateType,
        data: Vec<Self>,
    },
    Code {
        label: String,
        code: String,
    },
    PhantomNewLine,
}
