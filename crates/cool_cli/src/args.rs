use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    pub crate_name: String,
    pub crate_path: PathBuf,
}
