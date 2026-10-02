pub mod compiler;
pub mod data;
pub mod rdocx_decl;

use crate::compiler::lua::LuaStyle;
use crate::compiler::paragraph::Paragraph;
use crate::compiler::title::{self, Title};
use crate::compiler::{Compiler, paragraph};
use crate::data::parser::strict_mark;
use crate::data::{IntoParse, error::ErrorEditor};
use chumsky::Parser as ChumskyParser;
use clap::Parser;
use data::PreParseData;
use std::{fs::File, io::Read, path::PathBuf};
use tracing::{Level, error, info, warn};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "Strict mark")]
#[command(long_about = None)]
struct Cli {
    // #[arg(short, long)]
    // style: PathBuf,
    // #[arg(short, long)]
    // output_path: Option<PathBuf>,
    #[arg(last = true)]
    path: PathBuf,
}

fn main() {
    let filter = Targets::new()
        .with_target(env!("CARGO_PKG_NAME"), Level::DEBUG)
        .with_default(Level::INFO);

    tracing_subscriber::registry()
        .with(fmt::Layer::new())
        .with(filter)
        .init();

    let cli = Cli::parse();

    if cli.path.extension() != Some("sm".as_ref()) {
        warn!("This file has the wrong file extension or no file extension at all.");
    }
    let mut file_str = String::new();

    File::open(&cli.path)
        .expect(&format!("Can`t open file {:?}", cli.path))
        .read_to_string(&mut file_str)
        .expect(&format!("Can`t read file {:?}", cli.path));

    let result = PreParseData::Pre {
        data_str: file_str,
        block_editor: ErrorEditor::none(),
    }
    .parse(|s| strict_mark().parse(s));

    info!("parse: {:?}", cli.path,);

    match result {
        Ok((ast, errs)) => {
            if !errs.is_empty() {
                warn!(
                    "Parse error:\n{}\n",
                    errs.iter()
                        .map(|err| format!("\t{:?}", err))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
            info!("Result:\n{}", serde_json::to_string_pretty(&ast).unwrap());

            let compiler = Compiler {
                paragraph: vec![Paragraph {
                    data: paragraph::Data {
                        paragraph_type: data::ParagraphType::Default,
                    },
                    style: LuaStyle::Base(paragraph::Style {
                        rdocx_style: rdocx_decl::paragraph::Style::default(),
                    }),
                }],
                titles: vec![Title {
                    data: title::Data { level: 1 },
                    style: LuaStyle::Base(title::Style {
                        rdocx_style: rdocx_decl::paragraph::Style::default()
                            .alignment(Some(rdocx_decl::paragraph::Alignment::Center)),
                    }),
                }],
            };

            let (result, errs) = compiler.compile(&ast);
            result.to_rdocx().save("test.docx").unwrap();
            if !errs.is_empty() {
                warn!(
                    "Compile error:\n{}\n",
                    errs.iter()
                        .map(|err| format!("\t{:?}", err))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
        }
        Err(errs) => {
            error!(
                "Total Error: [\n{}\n]",
                errs.iter()
                    .map(|err| format!("\t{:?}", err))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        }
    }
}
