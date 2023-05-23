use crate::{CodeGenerator, MemoryValue, Value};
use cool_ast::DeclStmtAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_decl_stmt(&mut self, decl: &DeclStmtAst) {
        let binding = self.resolve[decl.binding_id];

        let value = match self.tys[binding.ty_id] {
            Some(decl_ty) => {
                let decl_ptr = self.util_gen_named_alloca(decl_ty, binding.symbol.as_str());
                let value = self.gen_expr(&decl.expr, Some(MemoryValue::new(decl_ptr, decl_ty)));

                if !decl.expr.is_aggregate() {
                    self.builder.build_store(
                        decl_ptr,
                        *self.gen_loaded_value(value).as_basic_value().unwrap(),
                    );
                }

                Value::memory(decl_ptr, decl_ty)
            }
            None => {
                self.gen_expr(&decl.expr, None);
                Value::Void
            }
        };

        self.bindings.insert(decl.binding_id, value);
    }
}
