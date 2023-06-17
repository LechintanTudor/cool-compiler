mod args;

use crate::args::Args;
use clap::Parser as _;
use colored::Colorize;
use cool_driver::{CompileErrorBundle, CompileOptions, Package};
use std::process::ExitCode;

fn compile(options: &CompileOptions) -> Result<(), (CompileErrorBundle, Package)> {
    let (codegen, mut resolve) = match cool_driver::p0_init("x86_64-unknown-linux-gnu") {
        Ok((codegen, resolve)) => (codegen, resolve),
        Err(errors) => return Err((errors, Package::default())),
    };
    println!("p0 - init:          success");

    let package = cool_driver::p1_parse(&mut resolve, options)?;
    println!("p1 - parse:         success");

    if let Err(errors) = cool_driver::p2_define_tys(&package, &mut resolve) {
        return Err((errors, package));
    }
    println!("p2 - define tys:    success");

    if let Err(errors) = cool_driver::p3_define_fn_tys(&package, &mut resolve) {
        return Err((errors, package));
    }
    println!("p3 - define fn tys: success");

    let package_ast = match cool_driver::p4_gen_ast(&package, &mut resolve) {
        Ok(package_ast) => package_ast,
        Err(errors) => return Err((errors, package)),
    };
    println!("p4 - gen ast:       success");

    let module = cool_driver::p5_gen_code(&package_ast, &codegen, &resolve, options);
    module.print_to_file("../programs/bin/main.ll").unwrap();
    println!("p5 - gen code:      success");

    Ok(())
}

fn main() -> ExitCode {
    let args = Args::parse();
    let options = CompileOptions {
        crate_name: args.crate_name,
        crate_root_file: args.crate_root_file,
    };

    let Err((errors_bundle, package)) = compile(&options) else {
        return ExitCode::SUCCESS;
    };

    println!();

    for error in errors_bundle.errors.iter() {
        match error.span() {
            Some(span) => {
                let (file, position) = package
                    .source_map
                    .get_file_and_position_from_offset(span.start);

                println!(
                    "{}: {}.\n -> '{}', line {}, column {}.\n",
                    "Error".red(),
                    error,
                    file.path.display(),
                    position.line,
                    position.column,
                );
            }
            None => {
                println!("{}: {}.", "Error".red(), error);
            }
        }
    }

    ExitCode::FAILURE
}
