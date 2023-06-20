use crate::{BuilderExt, CodeGenerator};
use cool_ast::ForLoopAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_for_loop(&mut self, stmt: &ForLoopAst) {
        // Initializer
        self.gen_decl_stmt(&stmt.decl);
        if self.builder.current_block_diverges() {
            return;
        }

        let cond_block = self.append_block_after_current_block();
        self.builder.build_unconditional_branch(cond_block);

        // Condition
        self.builder.position_at_end(cond_block);

        let cond_value = self.gen_loaded_expr(&stmt.cond);
        if self.builder.current_block_diverges() {
            return;
        }

        let cond_value = cond_value.into_basic_value().into_int_value();
        let cond_value = self.builder.build_bool(cond_value);

        let body_block = self.append_block_after(cond_block);
        let end_block = self.append_block_after(body_block);

        self.builder
            .build_conditional_branch(cond_value, body_block, end_block);

        // Body
        self.builder.position_at_end(body_block);
        self.gen_block_expr(&stmt.body);

        if self.builder.current_block_diverges() {
            self.builder.position_at_end(end_block);
            return;
        }

        let after_block = self.append_block_after_current_block();
        self.builder.build_unconditional_branch(after_block);

        // After
        self.builder.position_at_end(after_block);
        self.gen_stmt(&stmt.after);

        if !self.builder.current_block_diverges() {
            self.builder.build_unconditional_branch(cond_block);
        }

        self.builder.position_at_end(end_block);
    }
}
