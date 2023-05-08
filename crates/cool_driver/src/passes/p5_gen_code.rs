use crate::{CodeGeneratorContext, CompileOptions, CompileResult};
use cool_ast::PackageAst;
use cool_codegen::CodeGenerator;
use cool_resolve::ResolveContext;
use inkwell::module::Module;

pub fn p5_gen_code<'a>(
    package: &PackageAst,
    codegen: &'a CodeGeneratorContext,
    resolve: &'a ResolveContext,
    options: &CompileOptions,
) -> CompileResult<Module<'a>> {
    let codegen = CodeGenerator::new(
        &codegen.context,
        &codegen.target_triple,
        &codegen.target_data,
        resolve,
        &options.crate_name,
        options.crate_root_file.to_str().unwrap(),
    );

    Ok(codegen.gen_module(package))
}
