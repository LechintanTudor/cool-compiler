mod args;

use crate::args::*;
use clap::Parser as _;
use cool_driver::passes;
use cool_resolve::TyConfig;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let (parsed_crate, _context) =
        passes::p0_parse(&args.crate_name, &args.crate_path, TyConfig { ptr_size: 8 })?;

    println!("{:#?}", parsed_crate);
    Ok(())
}
