mod args;

use crate::args::*;
use clap::Parser as _;
use cool_driver::passes;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    passes::p0_parse(&args.name, &args.path)?;
    Ok(())
}
