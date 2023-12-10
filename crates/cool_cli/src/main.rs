mod args;

use crate::args::*;
use clap::Parser as _;
use cool_package::Package;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let package = toml::from_str::<Package>(
        &std::fs::read_to_string("../packages/libc/1.0.0/package.toml").unwrap(),
    )
    .unwrap();

    // let ty_config = TyConfig {
    //     i8_align: 1,
    //     i16_align: 2,
    //     i32_align: 4,
    //     i64_align: 8,
    //     i128_align: 8,
    //     f32_align: 4,
    //     f64_align: 8,
    //     ptr_size: 8,
    // };

    Ok(())
}
