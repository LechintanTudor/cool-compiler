use crate::{BuilderExt, CodeGenerator, LoadedValue, Value};
use cool_ast::ReturnStmtAst;
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_return_stmt(&mut self, stmt: &ReturnStmtAst) -> Value<'a> {
        let value = stmt
            .expr
            .as_ref()
            .map(|expr| self.gen_loaded_expr(expr))
            .unwrap_or(LoadedValue::Void);

        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let value = value.as_basic_value().map(|value| value as &dyn BasicValue);
        self.builder.build_return(value);
        Value::Void
    }
}
