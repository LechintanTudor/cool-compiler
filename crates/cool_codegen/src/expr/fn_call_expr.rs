use crate::{BuilderExt, CallableValue, CodeGenerator, LoadedValue};
use cool_ast::FnCallExprAst;
use inkwell::values::BasicMetadataValueEnum;

impl<'a> CodeGenerator<'a> {
    pub fn gen_fn_call_expr(&mut self, expr: &FnCallExprAst) -> LoadedValue<'a> {
        // Function
        let fn_value = {
            let fn_value = self.gen_loaded_expr(&expr.fn_expr);
            if self.builder.current_block_diverges() {
                return LoadedValue::Void;
            }

            fn_value.into_callable_value()
        };

        // Arguments
        let mut arg_values = Vec::<BasicMetadataValueEnum<'a>>::new();
        for arg_expr in expr.arg_exprs.iter() {
            let arg_value = self.gen_loaded_expr(arg_expr);
            if self.builder.current_block_diverges() {
                return LoadedValue::Void;
            }

            if let LoadedValue::Register(arg_value) = arg_value {
                arg_values.push(arg_value.into())
            }
        }

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
