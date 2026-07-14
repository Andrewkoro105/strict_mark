pub mod data;
use std::{fs::File, io::Read, path::PathBuf};

use crate::data::{IntoParse, error::ErrorEditor};
use crate::data::parser::strict_mark;
use chumsky::Parser as ChumskyParser;
use clap::Parser;
use data::PreParseData;
use tracing::{Level, error, info, warn};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(name = "Strict mark")]
#[command(long_about = None)]
struct Cli {
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
                    "Error:\n{}\n",
                    errs.iter()
                        .map(|err| format!("\t{:?}", err))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
            info!("Result:\n{}", serde_json::to_string_pretty(&ast).unwrap());
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
