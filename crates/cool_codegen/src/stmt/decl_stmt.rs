use crate::{CodeGenerator, MemoryValue};
use cool_ast::DeclStmtAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_decl_stmt(&mut self, decl: &DeclStmtAst) {
        let binding = self.resolve[decl.binding_id];

        let value = match self.tys[binding.ty_id] {
            Some(decl_ty) => {
                let decl_ptr = self.util_gen_named_alloca(decl_ty, binding.symbol.as_str());
                self.gen_expr(&decl.expr, Some(MemoryValue::new(decl_ptr, decl_ty)))
            }
            None => self.gen_expr(&decl.expr, None),
        };

        self.bindings.insert(decl.binding_id, value);
    }
}
