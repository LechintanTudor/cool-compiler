use crate::{BuilderExt, CodeGenerator};
use cool_ast::ContinueStmtAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_continue_stmt(&mut self, stmt: &ContinueStmtAst) {
        let first_frame_id = self.jump_block().first_frame_id;
        self.gen_defers(first_frame_id, stmt.frame_id);

        if !self.builder.current_block_diverges() {
            self.builder
                .build_unconditional_branch(self.jump_block().continue_block);
        }
    }
}
