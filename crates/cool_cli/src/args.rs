use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub crate_name: String,
    pub crate_root_file: PathBuf,
}
