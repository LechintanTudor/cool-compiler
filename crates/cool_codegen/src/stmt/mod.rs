mod assign_stmt;

pub use self::assign_stmt::*;
use crate::{AnyTypeEnumExt, AnyValueEnumExt, CodeGenerator, Value};
use cool_ast::{DeclStmtAst, StmtAst};
use inkwell::values::BasicValue;

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

    pub fn gen_decl_stmt(&mut self, decl: &DeclStmtAst) {
        let binding = self.resolve[decl.binding_id];

        if self.resolve.is_ty_id_zst(binding.ty_id) {
            return;
        }

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

        let ty = self.tys[binding.ty_id].into_basic_type();
        let pointer = alloca_builder.build_alloca(ty, binding.symbol.as_str());
        let alloca = pointer.as_instruction_value().unwrap();

        self.bindings
            .insert(decl.binding_id, Value::Lvalue { pointer, ty });
        self.last_alloca = Some(alloca);

        let value = self.gen_rvalue_expr(&decl.expr).unwrap().into_basic_value();
        self.builder.build_store(pointer, value);
    }
}
