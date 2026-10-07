pub mod lua;
pub mod paragraph;
pub mod text;
pub mod title;

use crate::{
    compiler::{lua::LuaStyle, paragraph::Paragraph, title::Title},
    data::{self, ParseData},
    rdocx_decl::{
        self, document, run, utils::{Color, Length}
    },
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use tracing::warn;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cash {}

pub(self) trait Compile<T, R = Option<Vec<rdocx_decl::document::Content>>> {
    fn compile(&self, ast: &T, cash: &mut Cash) -> R;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCompiler<S, D> {
    pub style: LuaStyle<S>,
    pub data: Option<D>,
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
    pub doc_style: document::Stile,
    pub titles: Vec<Title>,
    pub paragraph: Vec<Paragraph>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            doc_style: document::Stile::default(),
            paragraph: vec![Paragraph {
                data: Some(paragraph::Data {
                    paragraph_type: data::ParagraphType::Default,
                }),
                style: LuaStyle::Base(paragraph::Style {
                    rdocx_style: rdocx_decl::paragraph::Style::default(),
                }),
            }],
            titles: vec![
                Title {
                    data: Some(title::Data { level: 1 }),
                    style: LuaStyle::Base(title::Style {
                        rdocx_style: rdocx_decl::paragraph::Style::default()
                            .alignment(Some(rdocx_decl::paragraph::Alignment::Center))
                            .borders(Some(rdocx_decl::utils::border::Borders::All {
                                style: rdocx_decl::utils::border::BorderStyle::Dashed,
                                size: rdocx_decl::utils::Length::Pt(1.),
                                color: Color::Hex("#000000".into()),
                            }))
                            .base_run(Some(
                                run::Style::default()
                                    .size(Some(Length::Pt(20.)))
                                    .bold(Some(true)),
                            )),
                    }),
                },
                Title {
                    data: Some(title::Data { level: 2 }),
                    style: LuaStyle::Base(title::Style {
                        rdocx_style: rdocx_decl::paragraph::Style::default()
                            .alignment(Some(rdocx_decl::paragraph::Alignment::Center))
                            .base_run(Some(
                                run::Style::default()
                                    .size(Some(Length::Pt(20.)))
                                    .bold(Some(true)),
                            )),
                    }),
                },
                Title {
                    data: Some(title::Data { level: 3 }),
                    style: LuaStyle::Base(title::Style {
                        rdocx_style: rdocx_decl::paragraph::Style::default()
                            .base_run(Some(
                                run::Style::default()
                                    .size(Some(Length::Pt(15.)))
                                    .bold(Some(true)),
                            )),
                    }),
                },
                Title {
                    data: None,
                    style: LuaStyle::Base(title::Style {
                        rdocx_style: rdocx_decl::paragraph::Style::default()
                            .base_run(Some(
                                run::Style::default()
                                    .size(Some(Length::Pt(12.)))
                                    .bold(Some(true)),
                            )),
                    }),
                },
            ],
        }
    }
}

impl Compiler {
    pub fn load(path: &Path) -> Self {
        File::open(path)
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
            .and_then(|file| {
                serde_saphyr::from_reader::<_, Self>(file)
                    .map_err(|err| Box::new(err) as Box<dyn std::error::Error>)
            })
            .map_err(|err| {
                warn!("Load compiler error ({path:?}): {err:?}");
                err
            })
            .unwrap_or_default()
    }
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
        (rdocx_decl::document::Document { contents, stile: self.doc_style.clone() }, errs)
    }
}
