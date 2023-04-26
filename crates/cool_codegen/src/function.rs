use crate::CodeGenerator;
use cool_ast::{ExternFnAst, FnAst};
use inkwell::module::Linkage;

impl CodeGenerator<'_> {
    pub fn add_extern_fn(&mut self, extern_fn_ast: &ExternFnAst) {
        let fn_name = self
            .resolve
            .get_path_by_item_id(extern_fn_ast.item_id)
            .last()
            .as_str();

        let ty = self.tys[extern_fn_ast.ty_id].into_function_type();
        self.module
            .add_function(fn_name, ty, Some(Linkage::External));
    }

    pub fn add_fn(&mut self, _fn_ast: &FnAst) {}
}
