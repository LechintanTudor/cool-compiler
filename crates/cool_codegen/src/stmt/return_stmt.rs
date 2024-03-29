use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::ReturnStmtAst;
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_return_stmt(&mut self, stmt: &ReturnStmtAst) -> Value<'a> {
        let value = self.gen_loaded_expr(&stmt.expr);
        self.gen_return_defers(stmt.frame_id);

        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let value = value.as_ref().map(|value| value as &dyn BasicValue);
        self.builder.build_return(value);
        Value::Void
    }
}
