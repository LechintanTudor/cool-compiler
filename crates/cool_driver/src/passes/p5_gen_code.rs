use crate::{CodeGeneratorContext, CompileOptions};
use cool_ast::PackageAst;
use cool_codegen::CodeGenerator;
use cool_resolve::ResolveContext;
use inkwell::module::Module;

pub fn p5_gen_code<'a>(
    package: &'a PackageAst,
    codegen: &'a CodeGeneratorContext,
    resolve: &'a ResolveContext,
    options: &CompileOptions,
) -> Module<'a> {
    let codegen = CodeGenerator::new(
        &codegen.context,
        &codegen.target_triple,
        &codegen.target_data,
        package,
        resolve,
        &options.crate_name,
        options.crate_root_file.to_str().unwrap(),
    );

    codegen.gen_module()
}
