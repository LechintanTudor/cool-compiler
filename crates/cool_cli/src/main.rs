mod args;

use crate::args::Args;
use clap::Parser as _;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let package = cool_driver::parse_crate(&args.crate_name, &args.crate_root_file)?;
    let _module_asts = cool_driver::generate_ast(package)?;

    cool_codegen::codegen();
    Ok(())
}
