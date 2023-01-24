#![allow(dead_code)]

mod lexer;
mod parser;
mod symbol;
mod utils;

use crate::lexer::SourceFile;
use crate::symbol::SYMBOL_TABLE;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        eprintln!("Invalid number of arguments");
        return ExitCode::from(1);
    }

    let path = &args[0];
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("{}", error);
            return ExitCode::from(2);
        }
    };

    let source_file = SourceFile::from_name_and_source(path.clone(), source);

    println!("[LINE OFFSETS]");
    for offset in source_file.line_offsets.as_slice() {
        println!("{}", offset);
    }

    println!("\n[TOKENS]");
    for token in source_file.iter_lang_tokens() {
        println!("{}", token.kind);
    }

    println!("\n[SYMBOLS]");
    for symbols in SYMBOL_TABLE.read_inner().iter() {
        println!("{}", symbols);
    }

    ExitCode::SUCCESS
}
