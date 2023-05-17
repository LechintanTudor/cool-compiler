use crate::{CodeGenerator, Value};
use cool_ast::SubscriptExprAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_subscript_expr(&mut self, expr: &SubscriptExprAst) -> Value<'a> {
        let base = self.gen_expr(&expr.base);
        let index = self
            .gen_loaded_expr(&expr.index)
            .into_basic_value()
            .into_int_value();

        match base {
            Value::Void => Value::Void,
            Value::Memory { pointer, ty } => {
                let elem_ty = ty.into_array_type().get_element_type();
                let elem_pointer =
                    unsafe { self.builder.build_gep(elem_ty, pointer, &[index], "") };

                Value::Memory {
                    pointer: elem_pointer,
                    ty: elem_ty,
                }
            }
            Value::Register(array_value) => {
                let array_pointer = self.util_gen_alloca(array_value, "");

                let elem_ty = array_value.get_type().into_array_type().get_element_type();
                let elem_pointer =
                    unsafe { self.builder.build_gep(elem_ty, array_pointer, &[index], "") };

                Value::Memory {
                    pointer: elem_pointer,
                    ty: elem_ty,
                }
            }
            Value::Fn(_) => unreachable!(),
        }
    }
}
