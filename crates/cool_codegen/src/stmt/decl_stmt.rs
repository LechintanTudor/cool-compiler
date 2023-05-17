use crate::{CodeGenerator, Value};
use cool_ast::DeclStmtAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_decl_stmt(&mut self, decl: &DeclStmtAst) {
        let binding = self.resolve[decl.binding_id];

        if self.resolve.is_ty_id_zst(binding.ty_id) {
            return;
        }

        let value = self.gen_loaded_expr(&decl.expr).into_basic_value();
        let pointer = self.util_gen_alloca(value, binding.symbol.as_str());
        let ty = value.get_type();

        self.bindings
            .insert(decl.binding_id, Value::Memory { pointer, ty });
    }
}
