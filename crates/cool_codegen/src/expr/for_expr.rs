use crate::{BuilderExt, CodeGenerator, LoadedValue};
use cool_ast::ForExprAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_for_expr(&mut self, expr: &ForExprAst) -> LoadedValue<'a> {
        // Initializer
        self.gen_decl_stmt(&expr.decl);
        if self.builder.current_block_diverges() {
            return LoadedValue::Void;
        }

        let cond_block = self.append_block_after_current_block();
        self.builder.build_unconditional_branch(cond_block);

        // Condition
        self.builder.position_at_end(cond_block);

        let cond_value = self.gen_loaded_expr(&expr.cond);
        if self.builder.current_block_diverges() {
            return LoadedValue::Void;
        }

        let cond_value = cond_value.into_basic_value().into_int_value();
        let cond_value = self.builder.build_bool(cond_value);

        let body_block = self.append_block_after(cond_block);
        let end_block = self.append_block_after(body_block);

        self.builder
            .build_conditional_branch(cond_value, body_block, end_block);

        // Body
        self.builder.position_at_end(body_block);
        self.gen_block_expr(&expr.body);

        if self.builder.current_block_diverges() {
            self.builder.position_at_end(end_block);
            return LoadedValue::Void;
        }

        let after_block = self.append_block_after_current_block();
        self.builder.build_unconditional_branch(after_block);

        // After
        self.builder.position_at_end(after_block);
        self.gen_stmt(&expr.after);

        if !self.builder.current_block_diverges() {
            self.builder.build_unconditional_branch(cond_block);
        }

        self.builder.position_at_end(end_block);
        LoadedValue::Void
    }
}
