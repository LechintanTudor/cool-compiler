use crate::{AnyValueEnumExt, CodeGenerator, Value};
use cool_ast::{
    BindingExprAst, BlockExprAst, ExprAst, FnCallExprAst, LiteralExprAst, LiteralExprValue,
};
use inkwell::values::{AnyValue, AnyValueEnum};

impl<'a> CodeGenerator<'a> {
    pub fn gen_expr(&mut self, expr: &ExprAst) -> Value<'a> {
        match expr {
            ExprAst::Block(e) => self.gen_block_expr(e),
            ExprAst::FnCall(e) => self.gen_fn_call_expr(e),
            ExprAst::Binding(e) => self.gen_ident_expr(e),
            ExprAst::Literal(e) => self.gen_literal_expr(e).into(),
            ExprAst::Module(_) => panic!("module exprs are not allowed here"),
        }
    }

    pub fn gen_rvalue_expr(&mut self, expr: &ExprAst) -> Option<AnyValueEnum<'a>> {
        let value = self.gen_expr(expr);
        self.gen_loaded_value(value)
    }

    pub fn gen_block_expr(&mut self, expr: &BlockExprAst) -> Value<'a> {
        let Some((end_elem, start_elems)) = expr.elems.split_last() else {
            return Value::Void;
        };

        for elem in start_elems {
            self.gen_block_elem(elem);
        }

        self.gen_block_elem(end_elem)
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
                let ty_id = self.resolve[expr.expr_id];
                let int_ty = self.tys[ty_id].into_int_type();
                let parts = [
                    (value & (u64::MAX as u128)) as u64,
                    ((value >> 64) & (u64::MAX as u128)) as u64,
                ];

                int_ty.const_int_arbitrary_precision(&parts).into()
            }
            LiteralExprValue::Float(value) => {
                let ty_id = self.resolve[expr.expr_id];
                let float_ty = self.tys[ty_id].into_float_type();
                float_ty.const_float(*value).into()
            }
            LiteralExprValue::Cstr(value) => self
                .builder
                .build_global_string_ptr(&value, "")
                .as_any_value_enum(),
            _ => todo!(),
        }
    }

    pub fn gen_loaded_value(&self, value: Value<'a>) -> Option<AnyValueEnum<'a>> {
        match value {
            Value::Void => None,
            Value::Lvalue {
                pointer: address,
                ty,
            } => Some(self.builder.build_load(ty, address, "").as_any_value_enum()),
            Value::Rvalue(value) => Some(value),
        }
    }
}
