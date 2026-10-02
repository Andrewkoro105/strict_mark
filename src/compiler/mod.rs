pub mod lua;
pub mod paragraph;
pub mod text;
pub mod title;

use crate::{
    compiler::{lua::LuaStyle, paragraph::Paragraph, title::Title},
    data::ParseData,
    rdocx_decl,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cash {}

pub(self) trait Compile<T, R = Option<Vec<rdocx_decl::document::Content>>> {
    fn compile(&self, ast: &T, cash: &mut Cash) -> R;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base<S, D> {
    pub style: LuaStyle<S>,
    pub data: D,
}

#[derive(Debug, Clone)]
pub enum Error {
    NoSuitableStyle(ParseData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Integration {
    Url(String),
    Path(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compiler {
    pub titles: Vec<Title>,
    pub paragraph: Vec<Paragraph>,
}

impl Compile<ParseData, (Vec<rdocx_decl::document::Content>, Vec<Error>)> for Compiler {
    fn compile(
        &self,
        ast: &ParseData,
        cash: &mut Cash,
    ) -> (Vec<rdocx_decl::document::Content>, Vec<Error>) {
        match ast {
            ParseData::List(list) => {
                let (contents, errs) = list
                    .iter()
                    .map(|parse_data| {
                        <Compiler as Compile<
                            ParseData,
                            (Vec<rdocx_decl::document::Content>, Vec<Error>),
                        >>::compile(self, parse_data, cash)
                    })
                    .unzip::<_, _, Vec<_>, Vec<_>>();
                (contents.concat(), errs.concat())
            }
            ParseData::Title(title) => self
                .titles
                .iter()
                .find_map(|compiler| compiler.compile(title, cash))
                .map(|result| (result, vec![]))
                .unwrap_or_else(|| (vec![], vec![Error::NoSuitableStyle(ast.clone())])),
            ParseData::Paragraph(paragraph) => self
                .paragraph
                .iter()
                .find_map(|compiler| compiler.compile(paragraph, cash))
                .map(|result| (result, vec![]))
                .unwrap_or_else(|| (vec![], vec![Error::NoSuitableStyle(ast.clone())])),
            ParseData::PhantomNewLine => (vec![], vec![]),
            _ => todo!("Not supported: {ast:#?}"),
        }
    }
}

impl Compiler {
    pub fn compile(&self, ast: &ParseData) -> (rdocx_decl::document::Document, Vec<Error>) {
        let (contents, errs) = <Compiler as Compile<
            ParseData,
            (Vec<rdocx_decl::document::Content>, Vec<Error>),
        >>::compile(self, ast, &mut Cash::default());
        (rdocx_decl::document::Document { contents }, errs)
    }
}
