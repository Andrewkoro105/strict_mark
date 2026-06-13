pub mod data;
use std::{fs::File, io::Read, path::PathBuf};

use chumsky::Parser as ChumskyParser;
use clap::{Parser, builder::OsStr};
use tracing::{Level, info, warn};
use tracing_subscriber::{filter::Targets, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::data::parser::strict_mark_parser;

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

    let (ast, errors) = strict_mark_parser().parse(&file_str).into_output_errors();

    info!(
        "{:?} AST:\nError:\n{}\nResult:\n{}",
        cli.path,
        if !errors.is_empty() {
            errors
                .iter()
                .map(|err| format!("\t{:?}", err))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            "\tNone".to_string()
        },
        if let Some(ast) = ast {
            ast.iter()
                .map(|data| format!("\t{:?}", data))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            "\tNone".to_string()
        }
    );
}
