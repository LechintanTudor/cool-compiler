mod args;

use clap::Parser as _;
use cool_lexer::lexer::SourceFile;
use cool_lexer::symbols;
use cool_parser::parser::Parser;
use crate::args::Args;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let source = std::fs::read_to_string(args.crate_root_file)?;
    let source_file = SourceFile::from_name_and_source(args.crate_name, source);

    println!("[LINE OFFSETS]");
    for offset in source_file.line_offsets.as_slice() {
        println!("{}", offset);
    }
    println!();

    println!("[SYMBOLS]");
    for symbols in symbols::read_symbol_table().iter() {
        println!("{}", symbols);
    }
    println!();

    println!("[TOKENS]");
    for token in source_file.iter_lang_tokens() {
        println!("{}", token.kind);
    }
    println!();

    let mut parser = Parser::new(
        source_file.iter_lang_tokens(),
        source_file.source.len() as u32,
    );
    
    let module = parser.parse_module_item()
        .map_err(|error| {
            let line = source_file.line_offsets.to_line(error.span().start);
            anyhow::Error::new(error).context(format!("Parser error on line {}.", line))
        })?;

    println!("[MODULE]");
    println!("{:#?}", module);

    Ok(())
}
