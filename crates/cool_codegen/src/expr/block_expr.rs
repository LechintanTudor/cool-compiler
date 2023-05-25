use crate::{BuilderExt, CodeGenerator, LoadedValue};
use cool_ast::BlockExprAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_block_expr(&mut self, block: &BlockExprAst) -> LoadedValue<'a> {
        if self.builder.current_block_diverges() {
            return LoadedValue::Void;
        }

        for stmt in block.stmts.iter() {
            self.gen_stmt(stmt);
        }

        match block.expr.as_ref() {
            Some(expr) => self.gen_loaded_expr(expr),
            None => LoadedValue::Void,
        }
    }
}