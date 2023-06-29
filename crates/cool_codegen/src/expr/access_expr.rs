use crate::{BuilderExt, CodeGenerator, LoadedValue, Value};
use cool_ast::{AccessExprAst, ArrayLenExprAst};
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_access_expr(&mut self, expr: &AccessExprAst) -> Value<'a> {
        match self.gen_expr(&expr.base, None) {
            Value::Void => Value::Void,
            Value::Memory(memory) => {
                let base_ty_id = expr.base.expr_id().ty_id;

                let Some(field_index) = self
                    .tys
                    .get_field_map(base_ty_id)
                    .get(expr.ident.symbol) else {
                        return Value::Void;
                    };

                let field_ptr = self
                    .builder
                    .build_struct_gep(memory.ty, memory.ptr, field_index, "")
                    .unwrap();

                let field_ty_id = expr.expr_id.ty_id;
                let field_ty = self.tys[field_ty_id].unwrap();
                Value::memory(field_ptr, field_ty)
            }
            Value::Register(value) => {
                let base_ty_id = expr.base.expr_id().ty_id;

                let Some(field_index) = self
                    .tys
                    .get_field_map(base_ty_id)
                    .get(expr.ident.symbol) else {
                        return Value::Void;
                    };

                let field_value = self
                    .builder
                    .build_extract_value(value.into_struct_value(), field_index, "")
                    .unwrap();

                Value::Register(field_value)
            }
            Value::Fn(_) => unreachable!(),
        }
    }

    pub fn gen_array_len_expr(&mut self, expr: &ArrayLenExprAst) -> LoadedValue<'a> {
        self.gen_expr(&expr.base, None);

        if self.builder.current_block_diverges() {
            return LoadedValue::Void;
        }

        let array_len = expr.base.expr_id().ty_id.get_array().len;

        self.tys
            .isize_ty()
            .const_int(array_len, false)
            .as_basic_value_enum()
            .into()
    }
}
