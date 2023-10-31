mod args;

use crate::args::*;
use clap::Parser as _;
use cool_driver::passes;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    passes::p0_parse(&args.crate_name, &args.crate_path)?;
    Ok(())
}
