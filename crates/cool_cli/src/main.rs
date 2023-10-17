mod args;

use self::args::*;
use clap::Parser;

fn main() {
    Args::parse();
}
