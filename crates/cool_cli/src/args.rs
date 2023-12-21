use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    pub crates: Vec<PathBuf>,
}
