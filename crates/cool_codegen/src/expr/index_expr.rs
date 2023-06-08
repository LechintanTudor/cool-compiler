use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::IndexExprAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_index_expr(&mut self, expr: &IndexExprAst) -> Value<'a> {
        self.gen_array_index_expr(expr)
    }

    pub fn gen_array_index_expr(&mut self, expr: &IndexExprAst) -> Value<'a> {
        let base = self.gen_expr(&expr.base, None);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let index = self.gen_loaded_expr(&expr.index);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let index_value = index.as_basic_value().unwrap().into_int_value();

        match base {
            Value::Void => Value::Void,
            Value::Memory(memory) => {
                let elem_ty = memory.ty.into_array_type().get_element_type();
                let elem_ptr = unsafe {
                    self.builder
                        .build_gep(elem_ty, memory.ptr, &[index_value], "")
                };

                Value::memory(elem_ptr, elem_ty)
            }
            Value::Register(array_value) => {
                let array_ptr = self.util_gen_init(array_value);

                let elem_ty = array_value.get_type().into_array_type().get_element_type();
                let elem_ptr = unsafe {
                    self.builder
                        .build_gep(elem_ty, array_ptr, &[index_value], "")
                };

                Value::memory(elem_ptr, elem_ty)
            }
            Value::Fn(_) => unreachable!(),
        }
    }
}
