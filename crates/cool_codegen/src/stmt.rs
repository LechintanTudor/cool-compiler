use crate::{AnyTypeEnumExt, AnyValueEnumExt, CodeGenerator, Value};
use cool_ast::StmtAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_stmt(&mut self, stmt: &StmtAst) {
        match stmt {
            StmtAst::Decl(decl) => {
                let binding = self.resolve[decl.binding_id];

                if !self.resolve.is_ty_id_zst(binding.ty_id) {
                    let ty = self.tys[binding.ty_id].into_basic_type();
                    let pointer = self.builder.build_alloca(ty, "");

                    self.bindings
                        .insert(decl.binding_id, Value::Lvalue { pointer, ty });
                    let value = self.gen_rvalue_expr(&decl.expr).unwrap();
                    self.builder.build_store(pointer, value.into_basic_value());
                }
            }
            StmtAst::Expr(expr) => {
                self.gen_expr(expr);
            }
            _ => (),
        };
    }
}
