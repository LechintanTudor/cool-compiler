mod args;

use crate::args::Args;
use clap::Parser as _;
use cool_codegen::Codegen;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut package = cool_driver::parse_crate(&args.crate_name, &args.crate_root_file)?;
    let module_asts = cool_driver::generate_ast(&mut package)?;

    let context = cool_codegen::create_context();
    let codegen = Codegen::new(
        &context,
        "x86_64-unknown-linux-gnu",
        &package.items,
        &package.tys,
    );
    let module = codegen.run_for_module(&module_asts[0]);

    module.print_to_file("../programs/test.ll").unwrap();
    Ok(())
}
