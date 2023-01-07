mod lexer;
mod parser;
mod utils;

use crate::lexer::SourceFile;
use crate::parser::Parser;
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

    let source_file = match SourceFile::from_name_and_source(path.clone(), source) {
        Ok(source_file) => source_file,
        Err(error) => {
            eprintln!("{}", error);
            return ExitCode::from(3);
        }
    };

    // let root_ast = match Parser::new(&tokens, &idents, &literals).parse() {
    //     Ok(root_ast) => root_ast,
    //     Err(error) => {
    //         eprintln!("parser error: {}", error);
    //         return ExitCode::from(4);
    //     }
    // };

    println!("[TOKEN]");
    for token in source_file.spanned_tokens.iter() {
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
