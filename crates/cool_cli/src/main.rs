mod args;

use crate::args::Args;
use clap::Parser as _;
use cool_driver::CompileOptions;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let options = CompileOptions {
        crate_name: args.crate_name,
        crate_root_file: args.crate_root_file,
    };

    let (codegen, mut resolve) = cool_driver::p0_init("x86_64-unknown-linux-gnu")?;
    println!("p0 - init:          success");

    let package = cool_driver::p1_parse(&mut resolve, &options)?;
    println!("p1 - parse:         success");

    cool_driver::p2_define_tys(&package, &mut resolve)?;
    println!("p2 - define tys:    success");

    cool_driver::p3_define_fn_tys(&package, &mut resolve)?;
    println!("p3 - define fn tys: success");

    let package = cool_driver::p4_gen_ast(&package, &mut resolve)?;
    println!("p4 - gen ast:       success");

    let module = cool_driver::p5_gen_code(&package, &codegen, &resolve, &options)?;
    println!("p5 - gen code:      success");

    module.print_to_file("../programs/bin/main.ll").unwrap();
    Ok(())
}
