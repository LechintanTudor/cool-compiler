use crate::{BuilderExt, CodeGenerator, JumpBlock};
use cool_ast::WhileLoopAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_while_loop(&mut self, stmt: &WhileLoopAst) {
        let cond_block = self.append_block_after_current_block();
        self.builder.build_unconditional_branch(cond_block);

        // Condition
        self.builder.position_at_end(cond_block);

        let cond_value = {
            let cond_value = self.gen_loaded_expr(&stmt.block.cond);
            if self.builder.current_block_diverges() {
                return;
            }

            let cond_value = cond_value.into_basic_value().into_int_value();
            self.builder.build_bool(cond_value)
        };

        let body_block = self.append_block_after(cond_block);
        let end_block = self.append_block_after(body_block);
        self.builder
            .build_conditional_branch(cond_value, body_block, end_block);

        self.push_jump_block(JumpBlock {
            first_frame_id: stmt.block.expr.first_frame_id,
            break_block: end_block,
            continue_block: cond_block,
        });

        // Body
        self.builder.position_at_end(body_block);

        self.gen_block_expr(&stmt.block.expr);
        if !self.builder.current_block_diverges() {
            self.builder.build_unconditional_branch(cond_block);
        }

        self.builder.position_at_end(end_block);
        self.pop_jump_block();
    }
}
