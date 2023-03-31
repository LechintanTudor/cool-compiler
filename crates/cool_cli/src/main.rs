mod args;

use crate::args::Args;
use clap::Parser as _;
use cool_driver::{Driver, Package};
use cool_resolve::resolve::ResolveTable;
use cool_resolve::ty::TyTable;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut symbols = ResolveTable::with_builtins();
    let mut driver = Driver::new(&mut symbols, &args.crate_name, &args.crate_root_file);

    while driver.process_next_file_module() {
        // Empty
    }

    driver.resolve_imports().unwrap();

    let mut package = Package {
        sources: driver.into_source_files(),
        resolve: symbols,
        tys: TyTable::with_builtins(),
    };

    for source in package.sources.iter() {
        println!("[[[ {} ]]]", source.paths.path.display());
        println!("{:#?}", source.module);
        println!();
    }

    let _module_asts = cool_driver::generate_ast(&mut package).unwrap();
    println!("Ast generation success!");

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
