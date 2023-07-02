use crate::{BuilderExt, CodeGenerator, LoadedValue, Value};
use cool_ast::FnCallExprAst;
use inkwell::values::BasicMetadataValueEnum;

impl<'a> CodeGenerator<'a> {
    pub fn gen_fn_call_expr(&mut self, expr: &FnCallExprAst) -> LoadedValue<'a> {
        // Function
        let fn_value = self.gen_expr(&expr.fn_expr, None);
        if self.builder.current_block_diverges() {
            return LoadedValue::None;
        }

        // Arguments
        let mut arg_values = Vec::<BasicMetadataValueEnum<'a>>::new();
        for arg_expr in expr.arg_exprs.iter() {
            let arg_value = self.gen_loaded_expr(arg_expr);
            if self.builder.current_block_diverges() {
                return LoadedValue::None;
            }

            if let Some(arg_value) = arg_value {
                arg_values.push(arg_value.into())
            }
        }

        let ret_value = match fn_value {
            Value::Fn(fn_value) => {
                self.builder
                    .build_call(fn_value, &arg_values, "")
                    .try_as_basic_value()
            }
            Value::Register(value) => {
                let ty_id = expr.fn_expr.expr_id().ty_id;
                let fn_ty = self.tys.get_fn_ty(ty_id);
                let fn_pointer = value.into_pointer_value();

                self.builder
                    .build_indirect_call(fn_ty, fn_pointer, &arg_values, "")
                    .try_as_basic_value()
            }
            _ => panic!("value is not a function"),
        };

        ret_value
            .map_left(LoadedValue::Some)
            .left_or(LoadedValue::None)
    }
}
