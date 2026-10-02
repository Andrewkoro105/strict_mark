#[macro_use]
pub mod utils;
pub mod document;
pub mod paragraph;
pub mod run;

#[derive(Debug, Default)]
pub struct Cash {}

pub trait ToRdocx<D, T = ()> {
    fn to_rdocx(&self, doc: &mut D, data: T, cash: &mut Cash);
}

#[cfg(test)]
mod test {
    use crate::rdocx_decl::{
        document::Document,
        paragraph::{self, Paragraph},
        run::{self, Run},
        utils::Length,
    };
    use std::{fs, path::Path};

    #[test]
    fn simple() {
        let dir = Path::new("./tmp/test/");
        fs::create_dir_all(dir).unwrap();

        Document {
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
                            style: run::Style::default().all_caps(Some(true)),
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
        }
        .to_rdocx()
        .save(dir.join("test.docx"))
        .unwrap();
    }
}
