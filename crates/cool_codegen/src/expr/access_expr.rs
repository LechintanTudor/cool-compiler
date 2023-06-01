use crate::{BuilderExt, CodeGenerator, LoadedValue, Value};
use cool_ast::{ArrayAccessExprAst, StructAccessExprAst};
use cool_lexer::sym;
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_struct_access_expr(&mut self, expr: &StructAccessExprAst) -> Value<'a> {
        match self.gen_expr(&expr.base, None) {
            Value::Void => Value::Void,
            Value::Memory(memory) => {
                let base_ty_id = self.resolve[expr.base.expr_id()].ty_id;

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

                let field_ty_id = self.resolve[expr.expr_id].ty_id;
                let field_ty = self.tys[field_ty_id].unwrap();
                Value::memory(field_ptr, field_ty)
            }
            Value::Register(value) => {
                let base_ty_id = self.resolve[expr.base.expr_id()].ty_id;

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

    pub fn gen_array_access_expr(&mut self, expr: &ArrayAccessExprAst) -> LoadedValue<'a> {
        assert_eq!(expr.ident.symbol, sym::LEN);

        self.gen_expr(&expr.base, None);

        if self.builder.current_block_diverges() {
            return LoadedValue::Void;
        }

        let base_ty_id = self.resolve[expr.base.expr_id()].ty_id;
        let base_ty = self.resolve[base_ty_id].ty.as_array().unwrap();

        self.tys
            .isize_ty()
            .const_int(base_ty.len, false)
            .as_basic_value_enum()
            .into()
    }
}
