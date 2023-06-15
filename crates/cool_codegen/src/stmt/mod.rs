mod assign_stmt;
mod decl_stmt;
mod return_stmt;

use crate::{BuilderExt, CodeGenerator};
use cool_ast::StmtAst;
use inkwell::types::BasicType;
use inkwell::values::{BasicValue, PointerValue};

impl<'a> CodeGenerator<'a> {
    pub fn gen_stmt(&mut self, stmt: &StmtAst) {
        if self.builder.current_block_diverges() {
            return;
        }

        match stmt {
            StmtAst::Assign(assign) => {
                self.gen_assign_stmt(assign);
            }
            StmtAst::Decl(decl) => {
                self.gen_decl_stmt(decl);
            }
            StmtAst::Defer(_) => (),
            StmtAst::Expr(expr) => {
                self.gen_expr(expr, None);
            }
            StmtAst::Return(stmt) => {
                self.gen_return_stmt(stmt);
            }
        }
    }

    pub(crate) fn util_gen_alloca<T>(&mut self, ty: T) -> PointerValue<'a>
    where
        T: BasicType<'a>,
    {
        self.util_gen_named_alloca(ty, "")
    }

    pub(crate) fn util_gen_named_alloca<T>(&mut self, ty: T, name: &str) -> PointerValue<'a>
    where
        T: BasicType<'a>,
    {
        let alloca_builder = self.context.create_builder();
        let entry_block = self.get_fn_entry_block();

        match self.get_last_alloca() {
            Some(last_alloca) => {
                match last_alloca.get_next_instruction() {
                    Some(next_instruction) => {
                        alloca_builder.position_before(&next_instruction);
                    }
                    None => alloca_builder.position_at_end(entry_block),
                }
            }
            None => {
                match entry_block.get_first_instruction() {
                    Some(first_instruction) => alloca_builder.position_before(&first_instruction),
                    None => alloca_builder.position_at_end(entry_block),
                }
            }
        }

        let pointer = alloca_builder.build_alloca(ty, name);
        self.update_last_alloca(pointer.as_instruction_value().unwrap());
        pointer
    }

    pub(crate) fn util_gen_init<V>(&mut self, value: V) -> PointerValue<'a>
    where
        V: BasicValue<'a>,
    {
        let ptr = self.util_gen_alloca(value.as_basic_value_enum().get_type());
        self.builder.build_store(ptr, value);
        ptr
    }

    pub(crate) fn util_gen_named_init<V>(&mut self, value: V, name: &str) -> PointerValue<'a>
    where
        V: BasicValue<'a>,
    {
        let ptr = self.util_gen_named_alloca(value.as_basic_value_enum().get_type(), name);
        self.builder.build_store(ptr, value);
        ptr
    }
}
