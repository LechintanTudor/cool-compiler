use crate::{CodeGenerator, Value};
use cool_ast::DeclStmtAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_decl_stmt(&mut self, decl: &DeclStmtAst) {
        let binding = self.resolve[decl.binding_id];

        let value = match self.tys[binding.ty_id] {
            Some(decl_ty) => {
                let decl_ptr = self.util_gen_named_alloca(decl_ty, binding.symbol.as_str());
                let value = self.gen_expr(&decl.expr, Some(decl_ptr));

                if !decl.expr.uses_stack_memory() {
                    let ty_id = decl.expr.expr_id().ty_id;

                    if let Some(value) = self.gen_loaded_value(ty_id, value) {
                        self.builder.build_store(decl_ptr, value);
                    }
                }

                Value::Memory(decl_ptr)
            }
            None => {
                self.gen_expr(&decl.expr, None);
                Value::Void
            }
        };

        self.bindings.insert(decl.binding_id, value);
    }
}
