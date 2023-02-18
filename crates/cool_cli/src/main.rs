mod args;

use crate::args::Args;
use clap::Parser as _;
use cool_driver::SourceFile;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _source_file = SourceFile::from_path(&args.crate_name, args.crate_root_file)?;

    Ok(())
}
