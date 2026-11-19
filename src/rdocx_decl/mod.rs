#[macro_use]
pub mod utils;
pub mod document;
pub mod equation;
pub mod list_level;
pub mod paragraph;
pub mod run;

use std::{collections::HashMap, io, path::PathBuf};

use image::ImageError;
use rdocx::MathConversionDiagnostic;

use crate::rdocx_decl::list_level::ListLevelStyleId;

pub(crate) fn is_default<T: Default + PartialEq>(value: &T) -> bool {
    value == &T::default()
}

#[derive(Debug, Default)]
pub struct Cash {
    pub lists: HashMap<ListLevelStyleId, u32>,
    pub pictures: HashMap<String, Option<String>>,
}

#[derive(Debug)]
pub enum Error {
    SetNumberingValue,
    MathConversion(Vec<MathConversionDiagnostic>),
    AddEquation(rdocx::Error),
    ImageDecode(ImageError),
    ImageEncode(ImageError),
    ImageOpen(io::Error),
    UndefinedFileName { path: PathBuf },
    ImageNotFound { name: String },
}

pub trait ToRdocx<D, T = ()> {
    fn to_rdocx(&self, doc: &mut D, data: T, cash: &mut Cash) -> Vec<Error>;
}

#[cfg(test)]
mod test {
    use crate::rdocx_decl::{
        Error,
        document::{self, Document},
        paragraph::{self, Paragraph},
        run::{self, Caps, Run},
        utils::Length,
    };
    use std::{
        collections::HashMap,
        fs,
        path::{Path, PathBuf},
    };
    use sugar::hashmap;

    #[test]
    fn simple() -> Result<(), Error> {
        let dir = Path::new("./tmp/test/");
        fs::create_dir_all(dir).unwrap();

        let (mut doc, err) = Document {
            contents: vec![
                Paragraph {
                    contents: vec![
                        Run {
                            contents: vec![run::Content::Text("bibe ".into())],
                            style: run::Style::default(),
                        }
                        .into(),
                        Run {
                            contents: vec![run::Content::Text("babe ".into())],
                            style: run::Style::default().bold(Some(true)),
                        }
                        .into(),
                        Run {
                            contents: vec![run::Content::Text("gagi ".into())],
                            style: run::Style::default().italic(Some(true)),
                        }
                        .into(),
                        Run {
                            contents: vec![run::Content::Text("kika ".into())],
                            style: run::Style::default()
                                .underline_style(Some(run::UnderlineStyle::Dash)),
                        }
                        .into(),
                    ],
                    style: paragraph::Style::default()
                        .alignment(Some(paragraph::Alignment::Center)),
                }
                .into(),
                Paragraph {
                    contents: vec![
                        Run {
                            contents: vec![run::Content::Text("bibe ".into())],
                            style: run::Style::default().bold(Some(true)),
                        }
                        .into(),
                        Run {
                            contents: vec![run::Content::Text("babe ".into())],
                            style: run::Style::default(),
                        }
                        .into(),
                        Run {
                            contents: vec![run::Content::Text("gagi ".into())],
                            style: run::Style::default().caps(Some(Caps::All)),
                        }
                        .into(),
                        Run {
                            contents: vec![run::Content::Text("kika ".into())],
                            style: run::Style::default()
                                .underline_style(Some(run::UnderlineStyle::Words)),
                        }
                        .into(),
                        Run {
                            contents: vec![run::Content::Text("kika ".repeat(30))],
                            style: run::Style::default()
                                .underline_style(Some(run::UnderlineStyle::Words)),
                        }
                        .into(),
                    ],
                    style: paragraph::Style::default()
                        .alignment(Some(paragraph::Alignment::Right))
                        .signed_first_line_indent(Some(Length::Pt(-2.))),
                }
                .into(),
            ],
            stile: document::Stile::default(),
        }
        .to_rdocx();

        doc.save(dir.join("test.docx")).unwrap();
        assert!(err.is_empty());

        Ok(())
    }

    #[test]
    fn picture() {
        let dir = Path::new("./tmp/test/");
        fs::create_dir_all(dir).unwrap();

        let (mut doc, err) = Document {
            contents: vec![
                Paragraph {
                    contents: vec![
                        Run {
                            contents: vec![run::Content::Picture {
                                name: "name".to_owned(),
                                width: Length::Mm(50.),
                                height: Length::Mm(50.),
                            }],
                            style: run::Style::default(),
                        }
                        .into(),
                    ],
                    style: paragraph::Style::default()
                        .alignment(Some(paragraph::Alignment::Center)),
                }
                .into(),
            ],
            stile: document::Stile {
                lists: HashMap::new(),
                pictures: hashmap! {
                    "name".to_owned() => PathBuf::from("example_sm/my_demon.png"),
                },
            },
        }
        .to_rdocx();

        doc.save(dir.join("picture.docx")).unwrap();
        doc.save_pdf(dir.join("picture.pdf")).unwrap();
        assert!(err.is_empty());
    }
}
