mod args;

use crate::args::Args;
use clap::Parser as _;
use cool_driver::CompileOptions;
use cool_resolve::ResolveContext;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let options = CompileOptions {
        crate_name: args.crate_name,
        crate_root_file: args.crate_root_file,
    };

    let mut resolve = ResolveContext::with_builtins();
    let package = cool_driver::p0_parse(&mut resolve, &options)?;
    cool_driver::p1_define_tys(&package, &mut resolve)?;
    cool_driver::p2_gen_ast(&package, &mut resolve);

    // let _module_asts = cool_driver::generate_ast(&mut package).unwrap();
    // println!("Ast generation success!");

    // let context = cool_codegen::create_context();
    // let codegen = Codegen::new(
    //     &context,
    //     "x86_64-unknown-linux-gnu",
    //     &package.resolve,
    //     &package.tys,
    // );
    // let module = codegen.run_for_module(&module_asts[0]);

    // module.print_to_file("../programs/test.ll").unwrap();
    Ok(())
}
