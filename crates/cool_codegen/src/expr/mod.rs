mod access_expr;
mod array_expr;
mod binary_expr;
mod cond_expr;
mod fn_call_expr;
mod literal_expr;
mod return_expr;
mod struct_expr;
mod subscript_expr;
mod unary_expr;
mod while_expr;

use crate::{CodeGenerator, LoadedValue, MemoryValue, Value};
use cool_ast::{BindingExprAst, BlockExprAst, DerefExprAst, ExprAst};
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_expr(&mut self, expr: &ExprAst, memory: Option<MemoryValue<'a>>) -> Value<'a> {
        match expr {
            ExprAst::Array(e) => self.gen_array_expr(e, memory),
            ExprAst::ArrayRepeat(e) => self.gen_array_repeat_expr(e, memory),
            ExprAst::Binary(e) => self.gen_binary_expr(e).into(),
            ExprAst::Binding(e) => self.gen_ident_expr(e),
            ExprAst::Block(e) => self.gen_block_expr(e).into(),
            ExprAst::Cond(e) => self.gen_cond_expr(e).into(),
            ExprAst::Deref(e) => self.gen_deref_expr(e),
            ExprAst::FnCall(e) => self.gen_fn_call_expr(e).into(),
            ExprAst::Literal(e) => self.gen_literal_expr(e).into(),
            ExprAst::Return(e) => self.gen_return_expr(e),
            ExprAst::Struct(e) => self.gen_struct_expr(e, memory),
            ExprAst::StructAccess(e) => self.gen_struct_access_expr(e),
            ExprAst::Subscript(e) => self.gen_subscript_expr(e),
            ExprAst::Unary(e) => self.gen_unary_expr(e),
            ExprAst::While(e) => self.gen_while_expr(e).into(),
            _ => panic!("unsupported operation"),
        }
    }

    pub fn gen_loaded_expr(&mut self, expr: &ExprAst) -> LoadedValue<'a> {
        let value = self.gen_expr(expr, None);
        self.gen_loaded_value(value)
    }

    pub fn gen_block_expr(&mut self, block: &BlockExprAst) -> LoadedValue<'a> {
        for stmt in block.stmts.iter() {
            self.gen_stmt(stmt);
        }

        match block.expr.as_ref() {
            Some(expr) => self.gen_loaded_expr(expr),
            None => LoadedValue::Void,
        }
    }
    pub fn gen_ident_expr(&self, expr: &BindingExprAst) -> Value<'a> {
        self.bindings[&expr.binding_id]
    }

    #[inline]
    pub fn gen_deref_expr(&mut self, deref_expr: &DerefExprAst) -> Value<'a> {
        let pointer = self
            .gen_loaded_expr(&deref_expr.expr)
            .into_basic_value()
            .into_pointer_value();

        let expr_ty_id = self.resolve[deref_expr.expr_id].ty_id;
        let ty = self.tys[expr_ty_id].unwrap();

        Value::memory(pointer, ty)
    }

    pub fn gen_loaded_value(&self, value: Value<'a>) -> LoadedValue<'a> {
        match value {
            Value::Void => LoadedValue::Void,
            Value::Fn(fn_value) => {
                let value = fn_value
                    .as_global_value()
                    .as_pointer_value()
                    .as_basic_value_enum();

                LoadedValue::Register(value)
            }
            Value::Memory(memory) => {
                let value = self
                    .builder
                    .build_load(memory.ty, memory.ptr, "")
                    .as_basic_value_enum();

                LoadedValue::Register(value)
            }
            Value::Register(value) => LoadedValue::Register(value),
        }
    }
}
