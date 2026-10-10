#[macro_use]
pub mod utils;
pub mod document;
pub mod paragraph;
pub mod run;
pub mod list_level;

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Cash {
    pub lists: HashMap<String, u32>,
}

pub trait ToRdocx<D, T = ()> {
    fn to_rdocx(&self, doc: &mut D, data: T, cash: &mut Cash);
}

#[cfg(test)]
mod test {
    use crate::rdocx_decl::{
        document::{self, Document},
        paragraph::{self, Paragraph},
        run::{self, Caps, Run},
        utils::Length,
    };
    use rdocx::ListLevel;
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
            stile: document::Stile::default()
        }
        .to_rdocx()
        .save(dir.join("test.docx"))
        .unwrap();
    }

    #[test]
    fn list() {
        let dir = Path::new("./tmp/test/");
        fs::create_dir_all(dir).unwrap();

        let mut doc = rdocx::Document::new();
        let num_id = doc.add_list_definition(&[ListLevel::bullet(), ListLevel::decimal().start(3)]);
        doc.add_paragraph("first bullet").set_numbering(num_id, 0);
        doc.add_paragraph("third decimal").set_numbering(num_id, 1);
        doc.add_paragraph(
            "third decimal, third decimal, third decimal, third decimal,third decimal, ",
        );

        doc.add_paragraph("first bullet").set_numbering(num_id, 0);
        doc.add_paragraph("third decimal").set_numbering(num_id, 1);
    }
}
