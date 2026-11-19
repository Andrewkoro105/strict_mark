pub mod enumerate;
pub mod formula;
pub mod lua;
pub mod paragraph;
pub mod resource_insertion;
pub mod text;
pub mod title;

use crate::{
    compiler::{
        enumerate::Enumerate, formula::Formula, lua::LuaStyle, paragraph::Paragraph, resource_insertion::ResourceInsertion, title::Title
    },
    data::ParseData,
    rdocx_decl::{
        self, document::{self, StyleAdd},
        list_level::ListLevelStyleId,
        paragraph::{Alignment, ParagraphListLevelStyle},
        run,
        utils::{
            Color, Length,
            border::{self, Border},
        },
    },
};
use itertools::MultiUnzip;
use itertools::multiunzip;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use sugar::hashmap;
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

trait Compile<T, R = Option<Vec<rdocx_decl::document::Content>>> {
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
    pub doc_style: document::Style,
    pub titles: Vec<Title>,
    pub paragraphs: Vec<Paragraph>,
    pub enumerates: Vec<Enumerate>,
    pub formulas: Vec<Formula>,
    pub resource_insertion: Vec<ResourceInsertion>,
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            doc_style: document::Style::default(),
            paragraphs: vec![Paragraph {
                data: BaseCompilerData::none(),
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
                            .borders(hashmap! {
                                border::Edge::Top => Border {color: Color::Hex("#000000".into()),size: Length::Pt(15.),style: border::BorderStyle::DotDash,},
                                border::Edge::Bottom => Border {color: Color::Hex("#0000ff".into()),size: Length::Pt(15.),style: border::BorderStyle::DotDash,},
                                border::Edge::Left => Border {color: Color::Hex("#00ff00".into()),size: Length::Pt(15.),style: border::BorderStyle::DotDash,},
                                border::Edge::Right => Border {color: Color::Hex("#00ffff".into()),size: Length::Pt(15.),style: border::BorderStyle::DotDash,},
                                border::Edge::Between => Border {color: Color::Hex("#ff0000".into()),size: Length::Pt(15.),style: border::BorderStyle::DotDash,},
                                border::Edge::Bar => Border {color: Color::Hex("#ff00ff".into()),size: Length::Pt(15.),style: border::BorderStyle::DotDash,},
                            })
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
                    rdocx_style_name: ParagraphListLevelStyle {
                        id: ListLevelStyleId::unique(),
                        style: vec![]
                    },
                }
                .into(),
            }],
            formulas: vec![Formula{data: BaseCompilerData::none(), style: formula::Style{rdocx_style: rdocx_decl::paragraph::Style::default()}.into()}],
            resource_insertion: vec![
                ResourceInsertion{
                    data: BaseCompilerData::none(),
                    style: resource_insertion::Style{
                        rdocx_style_for_caption: rdocx_decl::paragraph::Style::default().alignment(Some(Alignment::Center)),
                        rdocx_style_for_image: rdocx_decl::paragraph::Style::default().alignment(Some(Alignment::Center)).keep_with_next(Some(true)).keep_together(Some(true)),
                        dpi: 300,
                    }.into()
                }
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

type ResultCompile = (
    Vec<rdocx_decl::document::Content>,
    Option<rdocx_decl::document::Style>,
    Vec<Error>,
);

impl Compile<ParseData, ResultCompile> for Compiler {
    fn compile(&self, ast: &ParseData, ctx: &Ctx, cash: &mut Cash) -> ResultCompile {
        match ast {
            ParseData::List(list) => {
                let (contents, style, errs): (Vec<_>, Vec<_>, Vec<_>) = list
                    .iter()
                    .map(|parse_data| {
                        <Compiler as Compile<ParseData, ResultCompile>>::compile(
                            self, parse_data, ctx, cash,
                        )
                    })
                    .multiunzip();
                (
                    contents.concat(),
                    style
                        .into_iter()
                        .fold(None, |result, style| result.add(style)),
                    errs.concat(),
                )
            }
            ParseData::Title(title) => self
                .titles
                .iter()
                .find_map(|compiler| compiler.compile(title, ctx, cash))
                .map(|result| (result, None, vec![]))
                .unwrap_or_else(|| (vec![], None, vec![Error::NoSuitableStyle(ast.clone())])),
            ParseData::Paragraph(paragraph) => self
                .paragraphs
                .iter()
                .find_map(|compiler| compiler.compile(paragraph, ctx, cash))
                .map(|result| (result, None, vec![]))
                .unwrap_or_else(|| (vec![], None, vec![Error::NoSuitableStyle(ast.clone())])),
            ParseData::PhantomNewLine => (vec![], None, vec![]),
            ParseData::Enumerate(enumerate) => self
                .enumerates
                .iter()
                .cloned()
                .find_map(|compiler| (compiler, self).compile(enumerate, ctx, cash))
                .unwrap_or_else(|| (vec![], None, vec![Error::NoSuitableStyle(ast.clone())])),
            ParseData::Formula(formula) => self
                .formulas
                .iter()
                .cloned()
                .find_map(|compiler| compiler.compile(formula, ctx, cash))
                .map(|result| (result, None, vec![]))
                .unwrap_or_else(|| (vec![], None, vec![Error::NoSuitableStyle(ast.clone())])),
            ParseData::Comments(_comments) => (vec![], None, vec![]),
            ParseData::ResourceInsertion(resource_insertion) => self
                .resource_insertion
                .iter()
                .cloned()
                .find_map(|compiler| compiler.compile(resource_insertion, ctx, cash))
                .map(|(result, (id, path))| (result, Some(rdocx_decl::document::Style{
                    pictures: hashmap! {
                        id => path,
                    },
                    ..Default::default()
                }), vec![]))
                .unwrap_or_else(|| (vec![], None, vec![Error::NoSuitableStyle(ast.clone())])),
            _ => todo!("Not supported: {ast:#?}"),
        }
    }
}

impl Compiler {
    pub fn compile(&self, ast: &ParseData) -> (rdocx_decl::document::Document, Vec<Error>) {
        let (contents, style, errs) = <Compiler as Compile<ParseData, ResultCompile>>::compile(
            self,
            ast,
            &Ctx { style_ctx: None },
            &mut Cash::default(),
        );
        (
            rdocx_decl::document::Document {
                contents,
                stile: style
                    .map(|style| self.doc_style.clone().add(style))
                    .unwrap_or(self.doc_style.clone()),
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
            .is_none_or(|(d, od)| *d == od);

        let ctx = self
            .ctx
            .as_ref()
            .zip(other.ctx.as_ref())
            .is_none_or(|(d, od)| *d == od);

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
