use crate::{BuilderExt, CodeGenerator, LoadedValue};
use cool_ast::WhileExprAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_while_expr(&mut self, expr: &WhileExprAst) -> LoadedValue<'a> {
        let cond_block = self.append_block_after_current_block();
        self.builder.build_unconditional_branch(cond_block);

        // Condition
        self.builder.position_at_end(cond_block);

        let cond_value = {
            let cond_value = self.gen_loaded_expr(&expr.block.cond);
            if self.builder.current_block_diverges() {
                return LoadedValue::Void;
            }

            let cond_value = cond_value.into_basic_value().into_int_value();
            self.builder.build_bool(cond_value)
        };

        let body_block = self.context.insert_basic_block_after(cond_block, "");
        let end_block = self.context.insert_basic_block_after(body_block, "");
        self.builder
            .build_conditional_branch(cond_value, body_block, end_block);

        // Body
        self.builder.position_at_end(body_block);

        self.gen_block_expr(&expr.block.expr);
        if self.builder.current_block_diverges() {
            self.builder.position_at_end(end_block);
            return LoadedValue::Void;
        }

        self.builder.build_unconditional_branch(cond_block);
        self.builder.position_at_end(end_block);
        LoadedValue::Void
    }
}
