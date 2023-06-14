use crate::{BuilderExt, CodeGenerator, LoadedValue};
use cool_ast::ForExprAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_for_expr(&mut self, expr: &ForExprAst) -> LoadedValue<'a> {
        self.gen_decl_stmt(&expr.decl);

        let start_block = self.builder.get_insert_block().unwrap();
        let cond_block = self.context.insert_basic_block_after(start_block, "");
        let body_block = self.context.insert_basic_block_after(cond_block, "");
        let after_block = self.context.insert_basic_block_after(body_block, "");
        let end_block = self.context.insert_basic_block_after(after_block, "");

        self.builder.build_unconditional_branch(cond_block);

        // Condition
        self.builder.position_at_end(cond_block);

        let cond_value = self
            .gen_loaded_expr(&expr.cond)
            .into_basic_value()
            .into_int_value();

        let cond_value = self.builder.build_bool(cond_value);
        self.builder
            .build_conditional_branch(cond_value, body_block, end_block);

        // Body
        self.builder.position_at_end(body_block);
        self.gen_block_expr(&expr.body);
        self.builder.build_unconditional_branch(after_block);

        // After
        self.builder.position_at_end(after_block);
        self.gen_stmt(&expr.after);
        self.builder.build_unconditional_branch(cond_block);

        self.builder.position_at_end(end_block);
        LoadedValue::Void
    }
}
