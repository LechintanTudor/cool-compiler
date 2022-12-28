mod lexer;
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

    let (tokens, identifiers, literals) = match lexer::tokenize(&source) {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("{}", error);
            return ExitCode::from(3);
        }
    };

    println!("[TOKEN]");
    for token in tokens.iter() {
        println!("{:?}", token);
    }

    println!("\n[IDENTIFIERS]");
    for identifier in identifiers.iter() {
        println!("{:?}", identifier);
    }

    println!("\n[LITERALS]");
    for literal in literals.iter() {
        println!("{:?}", literal);
    }

    ExitCode::SUCCESS
}
