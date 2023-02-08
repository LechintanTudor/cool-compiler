#![allow(dead_code)]

mod ast;
mod lexer;
mod parser;
mod symbol;
mod ty;
mod utils;

use crate::lexer::SourceFile;
use crate::parser::Parser;
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

    println!("\n[SYMBOLS]");
    for symbols in SYMBOL_TABLE.read_inner().iter() {
        println!("{}", symbols);
    }

    println!("\n[TOKENS]");
    for token in source_file.iter_lang_tokens() {
        println!("{}", token.kind);
    }

    let mut parser = Parser::new(
        source_file.iter_lang_tokens(),
        source_file.source.len() as u32,
    );
    let module = match parser.parse_module_item() {
        Ok(module) => module,
        Err(error) => {
            let line = source_file.line_offsets.to_line(error.span().start);
            eprintln!("\nError on line {}: {}", line, error);
            return ExitCode::from(3);
        }
    };

    println!("\n[MODULE]");
    println!("{:#?}", module);

    ExitCode::SUCCESS
}
