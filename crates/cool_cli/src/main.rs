mod args;

use self::args::*;
use clap::Parser as _;

fn main() {
    let args = Args::parse();
    let program = cool_driver::p0_parse(&args.name, &args.path);
    println!("{:#?}", program.files[0].file);
}
