mod scanner;

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

    let tokens = match scanner::tokenize(&source) {
        Ok(tokens) => tokens,
        Err(error) => {
            eprintln!("{}", error);
            return ExitCode::from(3);
        }
    };

    println!("{:?}", tokens);
    ExitCode::SUCCESS
}
