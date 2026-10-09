pub mod enumerate;
pub mod lua;
pub mod paragraph;
pub mod text;
pub mod title;

use crate::{
    compiler::{enumerate::Enumerate, lua::LuaStyle, paragraph::Paragraph, title::Title},
    data::{self, ParseData},
    rdocx_decl::{
        self, document,
        paragraph::ParagraphListLevelStyle,
        run,
        utils::{Color, Length},
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StyleCtx {}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ctx {
    style_ctx: Option<StyleCtx>,
    //enumerate_level: Option<(Option<String>, u32)>,
}

pub(self) trait Compile<T, R = Option<Vec<rdocx_decl::document::Content>>> {
    fn compile(&self, ast: &T, ctx: &Ctx, cash: &mut Cash) -> R;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCompilerData<D> {
    pub data: Option<D>,
    pub ctx: Option<Ctx>,
}

#[derive(Debug, Clone)]
pub struct BaseCompilerDataRef<'a, D> {
    pub data: Option<&'a D>,
    pub ctx: Option<&'a Ctx>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseCompiler<S, D> {
    pub style: LuaStyle<S>,
    pub data: BaseCompilerData<D>,
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
    pub paragraphs: Vec<Paragraph>,
    pub enumerates: Vec<Enumerate>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            doc_style: document::Stile::default(),
            paragraphs: vec![Paragraph {
                data: paragraph::Data {
                    paragraph_type: data::ParagraphType::Default,
                }
                .into(),
                style: paragraph::Style {
                    rdocx_style: rdocx_decl::paragraph::Style::default(),
                }
                .into(),
            }],
            titles: vec![
                Title {
                    data: title::Data { level: 1 }.into(),
                    style: title::Style {
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
                    }
                    .into(),
                },
                Title {
                    data: title::Data { level: 2 }.into(),
                    style: title::Style {
                        rdocx_style: rdocx_decl::paragraph::Style::default()
                            .alignment(Some(rdocx_decl::paragraph::Alignment::Center))
                            .base_run(Some(
                                run::Style::default()
                                    .size(Some(Length::Pt(20.)))
                                    .bold(Some(true)),
                            )),
                    }
                    .into(),
                },
                Title {
                    data: title::Data { level: 3 }.into(),
                    style: title::Style {
                        rdocx_style: rdocx_decl::paragraph::Style::default().base_run(Some(
                            run::Style::default()
                                .size(Some(Length::Pt(15.)))
                                .bold(Some(true)),
                        )),
                    }
                    .into(),
                },
                Title {
                    data: BaseCompilerData::none(),
                    style: title::Style {
                        rdocx_style: rdocx_decl::paragraph::Style::default().base_run(Some(
                            run::Style::default()
                                .size(Some(Length::Pt(12.)))
                                .bold(Some(true)),
                        )),
                    }
                    .into(),
                },
            ],
            enumerates: vec![Enumerate {
                data: BaseCompilerData::none(),
                style: enumerate::Style {
                    rdocx_style_name: ParagraphListLevelStyle::default(),
                }
                .into(),
            }],
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
        ctx: &Ctx,
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
                        >>::compile(self, parse_data, ctx, cash)
                    })
                    .unzip::<_, _, Vec<_>, Vec<_>>();
                (contents.concat(), errs.concat())
            }
            ParseData::Title(title) => self
                .titles
                .iter()
                .find_map(|compiler| compiler.compile(title, ctx, cash))
                .map(|result| (result, vec![]))
                .unwrap_or_else(|| (vec![], vec![Error::NoSuitableStyle(ast.clone())])),
            ParseData::Paragraph(paragraph) => self
                .paragraphs
                .iter()
                .find_map(|compiler| compiler.compile(paragraph, ctx, cash))
                .map(|result| (result, vec![]))
                .unwrap_or_else(|| (vec![], vec![Error::NoSuitableStyle(ast.clone())])),
            ParseData::PhantomNewLine => (vec![], vec![]),
            ParseData::Enumerate(enumerate) => self
                .enumerates
                .iter()
                .cloned()
                .find_map(|compiler| (compiler, self).compile(enumerate, ctx, cash))
                .unwrap_or_else(|| (vec![], vec![Error::NoSuitableStyle(ast.clone())])),
            _ => todo!("Not supported: {ast:#?}"),
        }
    }
}

impl Compiler {
    pub fn compile(&self, ast: &ParseData) -> (rdocx_decl::document::Document, Vec<Error>) {
        let (contents, errs) = <Compiler as Compile<
            ParseData,
            (Vec<rdocx_decl::document::Content>, Vec<Error>),
        >>::compile(
            self,
            ast,
            &Ctx {
                style_ctx: None,
                ..Default::default()
            },
            &mut Cash::default(),
        );
        (
            rdocx_decl::document::Document {
                contents,
                stile: self.doc_style.clone(),
            },
            errs,
        )
    }
}

impl<'a, D: PartialEq<D>> PartialEq<BaseCompilerData<D>> for BaseCompilerDataRef<'a, D> {
    fn eq(&self, other: &BaseCompilerData<D>) -> bool {
        let data = self
            .data
            .as_ref()
            .zip(other.data.as_ref())
            .map_or(true, |(d, od)| *d == od);

        let ctx = self
            .ctx
            .as_ref()
            .zip(other.ctx.as_ref())
            .map_or(true, |(d, od)| *d == od);

        data && ctx
    }
}

impl<'a, D> BaseCompilerDataRef<'a, D> {
    fn new(data: &'a D, ctx: &'a Ctx) -> Self {
        Self {
            data: Some(data),
            ctx: Some(ctx),
        }
    }
}

impl<D> BaseCompilerData<D> {
    fn none() -> Self {
        Self {
            data: None,
            ctx: None,
        }
    }
}

impl<D> From<D> for BaseCompilerData<D> {
    fn from(value: D) -> Self {
        Self {
            data: Some(value),
            ctx: Some(Ctx::default()),
        }
    }
}

impl Ctx {
    // pub fn add_enumerate_level(&mut self, new_name: Option<String>) {
    //     if let Some((name, level)) =  self.enumerate_level.as_mut() {
    //         *name = new_name;
    //         *level += 1;
    //     } else {
    //         self.enumerate_level = Some((new_name, 1))
    //     }
    // }
}
