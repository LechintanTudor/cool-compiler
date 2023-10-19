mod args;

use self::args::*;
use clap::Parser as _;
use cool_lexer::TokenStream;
use cool_parser::Parser;
use std::fs;

fn main() {
    let args = Args::parse();
    let source = fs::read_to_string(&args.file).unwrap();
    let tokens = TokenStream::new(&source);

    let mut parser = Parser::new(tokens);
    println!("{:#?}", parser.parse_block_expr().unwrap());
}
