use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::DerefExprAst;

impl<'a> CodeGenerator<'a> {
    #[inline]
    pub fn gen_deref_expr(&mut self, deref_expr: &DerefExprAst) -> Value<'a> {
        let pointer = self.gen_loaded_expr(&deref_expr.expr);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        Value::Memory(pointer.unwrap().into_pointer_value())
    }
}
