mod args;

use crate::args::Args;
use clap::Parser as _;
use colored::Colorize;
use cool_ast::PackageAst;
use cool_driver::{CompileError, CompileErrorBundle, CompileOptions, Package};
use std::process::ExitCode;

fn compile(options: &CompileOptions) -> Result<(), (Package, CompileErrorBundle)> {
    let (codegen, mut resolve) = match cool_driver::p0_init("x86_64-unknown-linux-gnu") {
        Ok((codegen, resolve)) => {
            println!("p0 - init:          success");
            (codegen, resolve)
        }
        Err(errors) => {
            println!("p0 - init:          error");
            return Err((Package::default(), errors));
        }
    };

    let (package, mut errors) = match cool_driver::p1_parse(&mut resolve, options) {
        Ok(package) => {
            println!("p1 - parse:         success");
            (package, vec![])
        }
        Err((package, error_bundle)) => {
            println!("p1 - parse:         error");
            (package, error_bundle.errors)
        }
    };

    match cool_driver::p2_define_tys(&package, &mut resolve) {
        Ok(_) => println!("p2 - define tys:    success"),
        Err(mut error_bundle) => {
            println!("p2 - define tys:    error");
            errors.append(&mut error_bundle.errors);
        }
    }

    match cool_driver::p3_define_fn_tys(&package, &mut resolve) {
        Ok(_) => println!("p3 - define fn tys: success"),
        Err(mut error_bundle) => {
            println!("p3 - define fn tys: error");
            errors.append(&mut error_bundle.errors);
        }
    }

    let package_ast = match cool_driver::p4_gen_ast(&package, &mut resolve) {
        Ok(package_ast) => {
            println!("p4 - gen ast:       success");
            package_ast
        }
        Err(mut error_bundle) => {
            println!("p4 - gen ast:       error");
            errors.append(&mut error_bundle.errors);
            PackageAst::default()
        }
    };

    if !errors.is_empty() {
        errors.sort_by_key(CompileError::span);
        return Err((package, CompileErrorBundle { errors }));
    }

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

    let Err((package, errors_bundle)) = compile(&options) else {
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
                println!("{}: {}.\n", "Error".red(), error);
            }
        }
    }

    ExitCode::FAILURE
}
