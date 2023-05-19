use crate::{BaiscTypeEnumOptionExt, CodeGenerator};
use cool_ast::{LiteralExprAst, LiteralExprValue};
use inkwell::values::{BasicValue, BasicValueEnum};

impl<'a> CodeGenerator<'a> {
    pub fn gen_literal_expr(&self, expr: &LiteralExprAst) -> BasicValueEnum<'a> {
        match &expr.value {
            LiteralExprValue::Int(value) => {
                let ty_id = self.resolve[expr.expr_id].ty_id;
                let int_ty = self.tys[ty_id].into_int_type();
                let parts = [
                    (value & (u64::MAX as u128)) as u64,
                    ((value >> 64) & (u64::MAX as u128)) as u64,
                ];

                int_ty.const_int_arbitrary_precision(&parts).into()
            }
            LiteralExprValue::Float(value) => {
                let ty_id = self.resolve[expr.expr_id].ty_id;
                let float_ty = self.tys[ty_id].into_float_type();
                float_ty.const_float(*value).into()
            }
            LiteralExprValue::Bool(value) => {
                let ty_id = self.resolve[expr.expr_id].ty_id;
                let bool_ty = self.tys[ty_id].into_int_type();
                bool_ty.const_int(*value as u64, false).into()
            }
            LiteralExprValue::Char(value) => {
                let ty_id = self.resolve[expr.expr_id].ty_id;
                let char_ty = self.tys[ty_id].into_int_type();
                char_ty.const_int(*value as u64, false).into()
            }
            LiteralExprValue::Cstr(value) => {
                self.builder
                    .build_global_string_ptr(value, "")
                    .as_basic_value_enum()
            }
        }
    }
}
