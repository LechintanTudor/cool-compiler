mod args;

use crate::args::Args;
use clap::Parser as _;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let package = cool_driver::compile(&args.crate_name, &args.crate_root_file);

    for path in package.items.iter_paths() {
        println!("{}", path);
    }

    Ok(())
}
