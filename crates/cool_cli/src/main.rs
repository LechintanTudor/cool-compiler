mod args;

use crate::args::Args;
use clap::Parser as _;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match cool_driver::compile(&args.crate_name, &args.crate_root_file) {
        Ok(package) => println!("Crate compiled successfully.\n {:#?}", package),
        Err(error) => println!("{}", error),
    }

    Ok(())
}
