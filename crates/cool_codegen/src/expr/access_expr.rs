use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::{AccessExprAst, ArrayLenExprAst};
use cool_lexer::Symbol;
use cool_resolve::TyId;
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_access_expr(&mut self, expr: &AccessExprAst) -> Value<'a> {
        let base_value = self.gen_expr(&expr.base, None);
        let base_ty_id = expr.base.expr_id().ty_id;
        self.util_gen_field_access(base_ty_id, base_value, expr.ident.symbol)
    }

    pub fn gen_array_len_expr(&mut self, expr: &ArrayLenExprAst) -> Value<'a> {
        self.gen_expr(&expr.base, None);

        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let array_len = expr.base.expr_id().ty_id.get_array().len;

        self.tys
            .isize_ty()
            .const_int(array_len, false)
            .as_basic_value_enum()
            .into()
    }

    pub fn util_gen_field_access(
        &mut self,
        struct_ty_id: TyId,
        struct_value: Value<'a>,
        field: Symbol,
    ) -> Value<'a> {
        let field_index = match self.tys.get_field_map(struct_ty_id).get(field) {
            Some(field_index) => field_index,
            None => return Value::Void,
        };

        match struct_value {
            Value::Register(value) => {
                self.builder
                    .build_extract_value(value.into_struct_value(), field_index, "")
                    .unwrap()
                    .into()
            }
            Value::Memory(memory) => {
                let struct_ty = match self.tys[struct_ty_id] {
                    Some(struct_ty) => struct_ty.into_struct_type(),
                    None => return Value::Void,
                };

                let field_ptr = self
                    .builder
                    .build_struct_gep(struct_ty, memory, field_index, "")
                    .unwrap();

                Value::Memory(field_ptr)
            }
            _ => Value::Void,
        }
    }
}
