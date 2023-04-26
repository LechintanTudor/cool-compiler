use crate::CodeGenerator;
use cool_ast::{
    BlockExprAst, ExprAst, FnCallExprAst, IdentExprAst, LiteralExprAst, LiteralExprValue,
};
use inkwell::values::{AnyValue, AnyValueEnum, BasicMetadataValueEnum, BasicValueEnum};

impl<'a> CodeGenerator<'a> {
    pub fn gen_expr(&mut self, expr: &ExprAst) -> Option<AnyValueEnum<'a>> {
        match expr {
            ExprAst::Block(e) => self.gen_block_expr(e),
            ExprAst::FnCall(e) => self.gen_fn_call_expr(e),
            ExprAst::Ident(e) => self.gen_ident_expr(e),
            ExprAst::Literal(e) => Some(self.gen_literal_expr(e)),
        }
    }

    pub fn gen_block_expr(&mut self, expr: &BlockExprAst) -> Option<AnyValueEnum<'a>> {
        let (end_elem, start_elems) = expr.elems.split_last()?;

        for elem in start_elems {
            self.gen_block_elem(elem);
        }

        self.gen_block_elem(end_elem)
    }

    fn gen_fn_call_expr(&mut self, expr: &FnCallExprAst) -> Option<AnyValueEnum<'a>> {
        let fn_expr = self.gen_expr(&expr.fn_expr).unwrap().into_function_value();

        let arg_exprs = expr
            .arg_exprs
            .iter()
            .flat_map(|arg| self.gen_expr(arg))
            .flat_map(basic_metadata_value_from_any_value_enum)
            .collect::<Vec<_>>();

        let ret_value = self
            .builder
            .build_call(fn_expr, &arg_exprs, "")
            .as_any_value_enum();

        Some(ret_value)
    }

    fn gen_ident_expr(&mut self, expr: &IdentExprAst) -> Option<AnyValueEnum<'a>> {
        let expr_ty_id = self.resolve[expr.binding_id].ty_id;

        if self.resolve.is_ty_id_zst(expr_ty_id) {
            return None;
        }

        Some(self.bindings[&expr.binding_id])
    }

    fn gen_literal_expr(&mut self, expr: &LiteralExprAst) -> AnyValueEnum<'a> {
        match &expr.value {
            LiteralExprValue::Int(value) => {
                let ty_id = self.resolve[expr.expr_id];
                let int_ty = self.tys[ty_id].into_int_type();
                int_ty.const_int(*value as u64, false).into()
            }
            LiteralExprValue::Cstr(value) => self
                .builder
                .build_global_string_ptr(&value, "")
                .as_any_value_enum(),
            _ => todo!(),
        }
    }
}

pub fn basic_metadata_value_from_any_value_enum<'a>(
    value: AnyValueEnum<'a>,
) -> Option<BasicMetadataValueEnum<'a>> {
    let value = match value {
        AnyValueEnum::ArrayValue(v) => BasicMetadataValueEnum::ArrayValue(v),
        AnyValueEnum::IntValue(v) => BasicMetadataValueEnum::IntValue(v),
        AnyValueEnum::FloatValue(v) => BasicMetadataValueEnum::FloatValue(v),
        AnyValueEnum::PointerValue(v) => BasicMetadataValueEnum::PointerValue(v),
        AnyValueEnum::StructValue(v) => BasicMetadataValueEnum::StructValue(v),
        AnyValueEnum::VectorValue(v) => BasicMetadataValueEnum::VectorValue(v),
        AnyValueEnum::MetadataValue(v) => BasicMetadataValueEnum::MetadataValue(v),
        _ => return None,
    };

    Some(value)
}

pub fn basic_value_from_any_value_enum<'a>(value: AnyValueEnum<'a>) -> Option<BasicValueEnum<'a>> {
    let value = match value {
        AnyValueEnum::ArrayValue(v) => BasicValueEnum::ArrayValue(v),
        AnyValueEnum::IntValue(v) => BasicValueEnum::IntValue(v),
        AnyValueEnum::FloatValue(v) => BasicValueEnum::FloatValue(v),
        AnyValueEnum::PointerValue(v) => BasicValueEnum::PointerValue(v),
        AnyValueEnum::StructValue(v) => BasicValueEnum::StructValue(v),
        AnyValueEnum::VectorValue(v) => BasicValueEnum::VectorValue(v),
        _ => return None,
    };

    Some(value)
}
