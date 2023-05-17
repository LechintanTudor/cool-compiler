mod array_expr;
mod binary_expr;
mod cond_expr;
mod struct_expr;
mod subscript_expr;
mod unary_expr;
mod while_expr;

pub use self::array_expr::*;
pub use self::binary_expr::*;
pub use self::cond_expr::*;
pub use self::struct_expr::*;
pub use self::subscript_expr::*;
pub use self::unary_expr::*;
pub use self::while_expr::*;
use crate::{AnyTypeEnumExt, AnyValueEnumExt, CodeGenerator, Value};
use cool_ast::{
    BindingExprAst, BlockExprAst, DerefExprAst, ExprAst, FnCallExprAst, LiteralExprAst,
    LiteralExprValue,
};
use inkwell::values::{AnyValue, AnyValueEnum};

impl<'a> CodeGenerator<'a> {
    pub fn gen_expr(&mut self, expr: &ExprAst) -> Value<'a> {
        match expr {
            ExprAst::Array(e) => self.gen_array_expr(e),
            ExprAst::ArrayRepeat(e) => self.gen_array_repeat_expr(e),
            ExprAst::Binary(e) => self.gen_binary_expr(e).into(),
            ExprAst::Binding(e) => self.gen_ident_expr(e),
            ExprAst::Block(e) => self.gen_block_expr(e),
            ExprAst::Cond(e) => self.gen_cond_expr(e),
            ExprAst::Deref(e) => self.gen_deref_expr(e),
            ExprAst::FnCall(e) => self.gen_fn_call_expr(e),
            ExprAst::Literal(e) => self.gen_literal_expr(e).into(),
            ExprAst::Subscript(e) => self.gen_subscript_expr(e),
            ExprAst::Unary(e) => self.gen_unary_expr(e),
            ExprAst::While(e) => self.gen_while_expr(e),
            _ => panic!("unsupported operation"),
        }
    }

    pub fn gen_rvalue_expr(&mut self, expr: &ExprAst) -> Option<AnyValueEnum<'a>> {
        let value = self.gen_expr(expr);
        self.gen_loaded_value(value)
    }

    pub fn gen_block_expr(&mut self, block: &BlockExprAst) -> Value<'a> {
        for stmt in block.stmts.iter() {
            self.gen_stmt(stmt);
        }

        match block.expr.as_ref() {
            Some(expr) => {
                self.gen_rvalue_expr(expr)
                    .map(Value::Rvalue)
                    .unwrap_or(Value::Void)
            }
            None => Value::Void,
        }
    }

    pub fn gen_fn_call_expr(&mut self, expr: &FnCallExprAst) -> Value<'a> {
        let fn_expr = self
            .gen_rvalue_expr(&expr.fn_expr)
            .unwrap()
            .into_function_value();

        let arg_exprs = expr
            .arg_exprs
            .iter()
            .flat_map(|arg| self.gen_rvalue_expr(arg))
            .flat_map(AnyValueEnumExt::try_into_basic_metadata_value)
            .collect::<Vec<_>>();

        let ret_value = self
            .builder
            .build_call(fn_expr, &arg_exprs, "")
            .as_any_value_enum()
            .into();

        ret_value
    }

    pub fn gen_ident_expr(&self, expr: &BindingExprAst) -> Value<'a> {
        self.bindings[&expr.binding_id]
    }

    pub fn gen_literal_expr(&self, expr: &LiteralExprAst) -> AnyValueEnum<'a> {
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
            LiteralExprValue::Cstr(value) => {
                self.builder
                    .build_global_string_ptr(value, "")
                    .as_any_value_enum()
            }
            _ => todo!(),
        }
    }

    #[inline]
    pub fn gen_deref_expr(&mut self, deref_expr: &DerefExprAst) -> Value<'a> {
        let pointer = self
            .gen_rvalue_expr(&deref_expr.expr)
            .unwrap()
            .into_pointer_value();

        let expr_ty_id = self.resolve[deref_expr.expr_id].ty_id;
        let ty = self.tys[expr_ty_id].into_basic_type();

        Value::Lvalue { pointer, ty }
    }

    pub fn gen_loaded_value(&self, value: Value<'a>) -> Option<AnyValueEnum<'a>> {
        match value {
            Value::Void => None,
            Value::Lvalue { pointer, ty } => {
                Some(self.builder.build_load(ty, pointer, "").as_any_value_enum())
            }
            Value::Rvalue(value) => Some(value),
        }
    }
}
