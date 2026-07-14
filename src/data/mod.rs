pub mod error;
pub mod parser;
#[macro_use]
pub mod pre_final_enums;

use crate::data::error::{Error, ErrorEditor};
use chumsky::{ParseResult, span::SimpleSpan};
use std::{collections::HashMap, path::PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum ParagraphType {
    #[default]
    Default,
    Text,
    Footnote,
    Other(String),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnumerateType {
    #[default]
    Default,
    Number,
    Mark,
    Char,
    Bibliography,
    Definitions,
    Other(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParamValues {
    List(Vec<(Self, SimpleSpan)>),
    Bool(bool),
    I32(i32),
    F32(f32),
    Value(String),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ParamType {
    List,
    Bool,
    I32,
    F32,
    Value,
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ParamData {
    pub value: ParamValues,
    pub name_span: SimpleSpan,
    pub value_span: Option<SimpleSpan>,
}

pub type Params = HashMap<String, ParamData>;

pre_final_enums!(
    "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]",
    ParseData, PreParseData, {
        List(Vec<Self>),
        Name {
            name: String,
            data: Box<Self>,
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
    },
    IntoParse<Error>::parse(parser: impl Fn(&String) -> ParseResult<PreParseData, Error> + Clone),
    {
        Pre {
            data_str: String,
            block_editor: ErrorEditor,
        } => {
            let (pre_result, mut pre_err) = parser(&data_str).into_output_errors();
            if let Some(pre_result) = pre_result {
                let (result, mut err) = pre_result
                    .parse(parser)
                    .map_err(|mut err| {
                        err.extend(pre_err.clone());
                        block_editor.edit_errs(&mut err);
                        err
                    })?;

                err.extend(pre_err);
                block_editor.edit_errs(&mut err);
                Ok((result, err))
            } else {
                block_editor.edit_errs(&mut pre_err);
                Err(pre_err)
            }
        },
    }
);
