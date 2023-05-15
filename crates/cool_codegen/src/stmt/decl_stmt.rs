use crate::{AnyValueEnumExt, CodeGenerator};
use cool_ast::DeclStmtAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_decl_stmt(&mut self, decl: &DeclStmtAst) {
        let binding = self.resolve[decl.binding_id];

        if self.resolve.is_ty_id_zst(binding.ty_id) {
            return;
        }

        let value = self.gen_rvalue_expr(&decl.expr).unwrap().into_basic_value();
        self.util_gen_alloca(value, binding.symbol.as_str());
    }
}
