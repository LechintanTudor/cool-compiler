use crate::{CompileOptions, CompileResult};
use cool_ast::PackageAst;
use cool_codegen::CodeGenerator;
use cool_resolve::ResolveContext;

pub fn p4_gen_code(
    package: &PackageAst,
    resolve: &ResolveContext,
    options: &CompileOptions,
) -> CompileResult<()> {
    let context = CodeGenerator::create_context();
    let codegen = CodeGenerator::new(
        &context,
        resolve,
        "x86_64-unknown-linux-gnu",
        &options.crate_name,
        options.crate_root_file.to_str().unwrap(),
    );

    let module = codegen.gen_module(package);
    module.print_to_file("../programs/bin/main.ll").unwrap();

    Ok(())
}
