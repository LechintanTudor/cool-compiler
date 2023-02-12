use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub crate_name: String,
    pub crate_root_file: PathBuf,
}
