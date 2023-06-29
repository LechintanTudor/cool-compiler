use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::DerefExprAst;

impl<'a> CodeGenerator<'a> {
    #[inline]
    pub fn gen_deref_expr(&mut self, deref_expr: &DerefExprAst) -> Value<'a> {
        let pointer = {
            let pointer = self.gen_loaded_expr(&deref_expr.expr);
            if self.builder.current_block_diverges() {
                return Value::Void;
            }

            pointer.into_basic_value().into_pointer_value()
        };

        let expr_ty_id = deref_expr.expr_id.ty_id;
        let ty = self.tys[expr_ty_id].unwrap();

        Value::memory(pointer, ty)
    }
}
