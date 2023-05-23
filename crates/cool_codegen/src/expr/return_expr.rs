use crate::{CodeGenerator, LoadedValue, Value};
use cool_ast::ReturnExprAst;
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_return_expr(&mut self, expr: &ReturnExprAst) -> Value<'a> {
        let value = expr
            .expr
            .as_ref()
            .map(|expr| self.gen_loaded_expr(expr))
            .unwrap_or(LoadedValue::Void);

        let value = value.as_basic_value().map(|value| value as &dyn BasicValue);
        self.builder.build_return(value);

        Value::Void
    }
}
