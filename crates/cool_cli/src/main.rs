mod args;

use crate::args::Args;
use clap::Parser as _;
use cool_driver::SourceFile;
use cool_parser::ParseTree;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let source_file = SourceFile::from_path(args.crate_root_file)?;

    for decl in source_file.module.decls.iter() {
        let ident = source_file.span_content(decl.ident_span);
        let item = source_file.span_content(decl.item.span());
        println!("--> {}\n{}\n", ident, item);
    }

    Ok(())
}
