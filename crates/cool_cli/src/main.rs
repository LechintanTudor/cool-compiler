mod args;

use crate::args::*;
use clap::Parser as _;
use cool_driver::p0_parse;
use cool_resolve::{ResolveContext, TyConfig};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let ty_config = TyConfig {
        i8_align: 1,
        i16_align: 2,
        i32_align: 4,
        i64_align: 8,
        i128_align: 8,
        f32_align: 4,
        f64_align: 8,
        ptr_size: 8,
    };

    let mut context = ResolveContext::new(ty_config);
    p0_parse(Default::default(), &mut context);

    Ok(())
}
