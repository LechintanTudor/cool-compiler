use crate::{CallableValue, CodeGenerator, LoadedValue};
use cool_ast::FnCallExprAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_fn_call_expr(&mut self, expr: &FnCallExprAst) -> LoadedValue<'a> {
        let fn_value = self.gen_loaded_expr(&expr.fn_expr).into_callable_value();

        let arg_values = expr
            .arg_exprs
            .iter()
            .map(|arg| self.gen_loaded_expr(arg).into_basic_value().into())
            .collect::<Vec<_>>();

        let ret_value = match fn_value {
            CallableValue::Fn(fn_value) => {
                self.builder
                    .build_call(fn_value, &arg_values, "")
                    .try_as_basic_value()
                    .map_left(LoadedValue::Register)
                    .left_or(LoadedValue::Void)
            }
            CallableValue::Register(fn_pointer) => {
                let ty_id = self.resolve[expr.fn_expr.expr_id()].ty_id;
                let fn_type = self.tys.get_fn_ty(ty_id);

                self.builder
                    .build_indirect_call(fn_type, fn_pointer, &arg_values, "")
                    .try_as_basic_value()
                    .map_left(LoadedValue::Register)
                    .left_or(LoadedValue::Void)
            }
        };

        ret_value
    }
}
