mod assign_stmt;
mod decl_stmt;

pub use self::assign_stmt::*;
pub use self::decl_stmt::*;
use crate::CodeGenerator;
use cool_ast::StmtAst;
use inkwell::values::{BasicValue, PointerValue};

impl<'a> CodeGenerator<'a> {
    pub fn gen_stmt(&mut self, stmt: &StmtAst) {
        match stmt {
            StmtAst::Assign(assign) => {
                self.gen_assign_stmt(assign);
            }
            StmtAst::Decl(decl) => {
                self.gen_decl_stmt(decl);
            }
            StmtAst::Expr(expr) => {
                self.gen_expr(expr);
            }
        }
    }

    pub(crate) fn util_gen_alloca<V>(&mut self, value: V, name: &str) -> PointerValue<'a>
    where
        V: BasicValue<'a>,
    {
        let alloca_builder = self.context.create_builder();
        let entry_block = self.fn_value.unwrap().get_first_basic_block().unwrap();

        match self.last_alloca.as_ref() {
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

        let ty = value.as_basic_value_enum().get_type();
        let pointer = alloca_builder.build_alloca(ty, name);

        self.last_alloca = Some(pointer.as_instruction_value().unwrap());
        self.builder.build_store(pointer, value);
        pointer
    }
}
