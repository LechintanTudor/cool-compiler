mod access_expr;
mod array_expr;
mod binary_expr;
mod block_expr;
mod cast_expr;
mod cond_expr;
mod deref_expr;
mod fn_call_expr;
mod index_expr;
mod layout_expr;
mod literal_expr;
mod match_expr;
mod range_expr;
mod struct_expr;
mod tuple_expr;
mod unary_expr;
mod variant_wrap_expr;

use crate::{BuilderExt, CodeGenerator, LoadedValue, Value};
use cool_ast::{BindingExprAst, ExprAst};
use cool_lexer::Symbol;
use cool_resolve::TyId;
use inkwell::values::{BasicValue, BasicValueEnum, PointerValue};

impl<'a> CodeGenerator<'a> {
    pub fn gen_expr(&mut self, expr: &ExprAst, memory: Option<PointerValue<'a>>) -> Value<'a> {
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        match expr {
            ExprAst::Access(e) => self.gen_access_expr(e),
            ExprAst::AlignOf(e) => self.gen_align_of_expr(e).as_basic_value_enum().into(),
            ExprAst::Array(e) => self.gen_array_expr(e, memory),
            ExprAst::ArrayLen(e) => self.gen_array_len_expr(e).into(),
            ExprAst::ArrayRepeat(e) => self.gen_array_repeat_expr(e, memory),
            ExprAst::Binary(e) => self.gen_binary_expr(e).into(),
            ExprAst::Binding(e) => self.gen_ident_expr(e),
            ExprAst::Block(e) => self.gen_block_expr(e).into(),
            ExprAst::Cast(e) => self.gen_cast_expr(e).into(),
            ExprAst::Cond(e) => self.gen_cond_expr(e).into(),
            ExprAst::Deref(e) => self.gen_deref_expr(e),
            ExprAst::FnCall(e) => self.gen_fn_call_expr(e).into(),
            ExprAst::Index(e) => self.gen_index_expr(e),
            ExprAst::Literal(e) => self.gen_literal_expr(e).into(),
            ExprAst::Match(e) => self.gen_match_expr(e).into(),
            ExprAst::OffsetOf(e) => self.gen_offset_of_expr(e).as_basic_value_enum().into(),
            ExprAst::Range(e) => self.gen_range_expr(e, memory),
            ExprAst::SizeOf(e) => self.gen_size_of_expr(e).as_basic_value_enum().into(),
            ExprAst::Struct(e) => self.gen_struct_expr(e, memory),
            ExprAst::Tuple(e) => self.gen_tuple_expr(e, memory),
            ExprAst::Unary(e) => self.gen_unary_expr(e),
            ExprAst::VariantWrap(e) => self.gen_variant_wrap_expr(e, memory),
            _ => panic!("unsupported codegen operation: {:#?}", expr),
        }
    }

    #[inline]
    pub fn gen_loaded_expr(&mut self, expr: &ExprAst) -> LoadedValue<'a> {
        let value = self.gen_expr(expr, None);
        self.gen_loaded_value(expr.expr_id().ty_id, value)
    }

    #[inline]
    pub fn gen_ident_expr(&self, expr: &BindingExprAst) -> Value<'a> {
        self.bindings[&expr.binding_id]
    }

    pub fn util_gen_loaded_field(
        &self,
        aggregate_ty_id: TyId,
        memory: PointerValue<'a>,
        field: Symbol,
    ) -> Option<BasicValueEnum<'a>> {
        let aggregate_ty = self.tys[aggregate_ty_id]?;
        let field_index = self.tys.get_field_map(aggregate_ty_id).get(field)?;

        let field_ptr = self
            .builder
            .build_struct_gep(aggregate_ty, memory, field_index, "")
            .unwrap();

        let field_ty_id = self
            .resolve
            .get_ty_def(aggregate_ty_id)
            .unwrap()
            .get_aggregate_field(field)
            .unwrap()
            .ty_id;

        let field_ty = self.tys[field_ty_id]?;

        self.builder.build_load(field_ty, field_ptr, "").into()
    }
}
