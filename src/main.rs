#![allow(dead_code)]

mod lexer;
mod parser2;
mod symbol;
mod utils;

use crate::lexer::SourceFile;
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

    // let root_ast = match Parser::new(&tokens, &source_file.idents, &source_file.literals).parse() {
    //     Ok(root_ast) => root_ast,
    //     Err(error) => {
    //         eprintln!("parser error: {}", error);
    //         return ExitCode::from(4);
    //     }
    // };

    println!("[LINE OFFSETS]");
    for offset in source_file.line_offsets.as_slice() {
        println!("{}", offset);
    }

    println!("\n[TOKENS]");
    for token in source_file.iter_lang_tokens() {
        println!("{}", token.kind);
    }

    println!("\n[SYMBOLS]");
    for symbols in source_file.symbols.iter() {
        println!("{}", symbols);
    }

    // println!("\n[ROOT AST]");
    // println!("{:#?}", root_ast);

    ExitCode::SUCCESS
}
