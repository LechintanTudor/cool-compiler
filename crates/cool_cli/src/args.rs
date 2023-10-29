use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub name: String,
    pub path: String,
}
