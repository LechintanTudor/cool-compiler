use crate::{BuilderExt, CodeGenerator, LoadedValue};
use cool_ast::BlockExprAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_block_expr(&mut self, block: &BlockExprAst) -> LoadedValue<'a> {
        if self.builder.current_block_diverges() {
            return LoadedValue::None;
        }

        for stmt in block.stmts.iter() {
            self.gen_stmt(stmt);

            if self.builder.current_block_diverges() {
                return LoadedValue::None;
            }
        }

        let value = self.gen_loaded_expr(&block.expr);

        self.gen_defers(block.first_frame_id, block.last_frame_id);

        if self.builder.current_block_diverges() {
            return LoadedValue::None;
        };

        value
    }
}
