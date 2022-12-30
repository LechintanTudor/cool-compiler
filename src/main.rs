mod lexer;
mod parser;
use crate::parser::Parser;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    if args.len() != 1 {
        eprintln!("Invalid number of arguments");
        return ExitCode::from(1);
    }

    let path = &args[0];
    let source = match std::fs::read(path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("{}", error);
            return ExitCode::from(2);
        }
    };

    let (tokens, idents, literals) = match lexer::tokenize(&source) {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("lexer error: {}", error);
            return ExitCode::from(3);
        }
    };

    let root_ast = match Parser::new(&tokens, &idents, &literals).parse() {
        Ok(root_ast) => root_ast,
        Err(error) => {
            eprintln!("parser error: {}", error);
            return ExitCode::from(4);
        }
    };

    println!("[TOKEN]");
    for token in tokens.iter() {
        println!("{:?}", token);
    }

    println!("\n[IDENTIFIERS]");
    for ident in idents.iter() {
        println!("{:?}", ident);
    }

    println!("\n[LITERALS]");
    for literal in literals.iter() {
        println!("{:?}", literal);
    }

    println!("\n[ROOT AST]");
    println!("{:#?}", root_ast);

    ExitCode::SUCCESS
}
