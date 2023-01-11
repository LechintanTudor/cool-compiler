#![allow(dead_code)]

mod lexer;
mod parser;
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

    // let tokens = source_file
    //     .tokens
    //     .iter()
    //     .map(|token| token.kind.clone())
    //     .collect::<Vec<_>>();

    // let root_ast = match Parser::new(&tokens, &source_file.idents, &source_file.literals).parse() {
    //     Ok(root_ast) => root_ast,
    //     Err(error) => {
    //         eprintln!("parser error: {}", error);
    //         return ExitCode::from(4);
    //     }
    // };

    println!("[LINE OFFSETS]");
    for offset in source_file.line_offsets.as_slice() {
        println!("{:?}", offset);
    }

    println!("[TOKEN]");
    for token in source_file.iter_tokens() {
        println!("{:?}", token);
    }

    println!("\n[IDENTIFIERS]");
    for ident in source_file.idents.iter() {
        println!("{:?}", ident);
    }

    println!("\n[LITERALS]");
    for literal in source_file.literals.iter() {
        println!("{:?}", literal);
    }

    // println!("\n[ROOT AST]");
    // println!("{:#?}", root_ast);

    ExitCode::SUCCESS
}
