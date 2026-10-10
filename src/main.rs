pub mod compiler;
pub mod data;
pub mod rdocx_decl;

use crate::compiler::Compiler;
use crate::data::parser::strict_mark;
use crate::data::{IntoParse, error::ErrorEditor};
use chumsky::Parser as ChumskyParser;
use clap::Parser;
use data::PreParseData;
use std::time::Instant;
use std::{fs::File, io::Read, path::PathBuf};
use tracing::{Level, debug, error, info, warn};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "Strict mark")]
#[command(long_about = None)]
struct BaseCli {
    #[arg(short, long)]
    style_path: Option<PathBuf>,
    #[arg(short, long)]
    output_path: Option<PathBuf>,
    #[arg(short, long)]
    pdf: bool,
    #[arg(short, long)]
    debug: bool,
    #[arg(last = true)]
    path: Option<PathBuf>,
}

struct Cli {
    style_path: PathBuf,
    output_path: PathBuf,
    pdf: bool,
    debug: bool,
    path: PathBuf,
}

impl From<BaseCli> for Cli {
    fn from(cli: BaseCli) -> Self {
        let path = cli.path.unwrap_or(PathBuf::from("main.sm"));
        Cli {
            style_path: cli.style_path.unwrap_or(PathBuf::from("style.yaml")),
            output_path: path.with_extension("docx"),
            pdf: cli.pdf,
            debug: cli.debug,
            path,
        }
    }
}

fn main() {
    let start = Instant::now();
    let cli = Cli::from(BaseCli::parse());

    let filter = Targets::new()
        .with_target(
            env!("CARGO_PKG_NAME"),
            if cli.debug { Level::DEBUG } else { Level::INFO },
        )
        .with_default(Level::INFO);

    tracing_subscriber::registry()
        .with(fmt::Layer::new())
        .with(filter)
        .init();

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
            debug!("Parse result:\n{:#?}", ast);

            let compiler = Compiler::load(&cli.style_path);

            let (rdocx_decl_ast, errs) = compiler.compile(&ast);
            debug!("Compile result:\n{}", serde_saphyr::to_string(&rdocx_decl_ast).unwrap());

            let mut rdocx_doc = rdocx_decl_ast.to_rdocx();
            rdocx_doc.save(&cli.output_path).unwrap();
            info!("Save: {:?}", cli.output_path);

            if cli.pdf {
                rdocx_doc.save_pdf(cli.output_path.with_extension("pdf")).unwrap();
                info!("Save: {:?}", cli.output_path);
            }


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

    info!("end: {:?}", start.elapsed());
}
