use crate::{CodeGenerator, Value};
use cool_ast::DerefExprAst;

impl<'a> CodeGenerator<'a> {
    #[inline]
    pub fn gen_deref_expr(&mut self, deref_expr: &DerefExprAst) -> Value<'a> {
        let pointer = self
            .gen_loaded_expr(&deref_expr.expr)
            .into_basic_value()
            .into_pointer_value();

        let expr_ty_id = self.resolve[deref_expr.expr_id].ty_id;
        let ty = self.tys[expr_ty_id].unwrap();

        Value::memory(pointer, ty)
    }
}