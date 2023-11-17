mod args;

use crate::args::*;
use clap::Parser as _;
use cool_driver::passes;
use cool_resolve::TyConfig;

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

    let (parsed_crate, mut context) =
        passes::p0_parse(&args.crate_name, &args.crate_path, ty_config)?;

    let defined_crate = passes::p1_define_items(parsed_crate, &mut context)?;
    passes::p2_generate_ast(&defined_crate, &mut context)?;

    Ok(())
}
