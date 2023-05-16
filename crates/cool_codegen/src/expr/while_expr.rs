use crate::{CodeGenerator, Value};
use cool_ast::WhileExprAst;
use inkwell::IntPredicate;

impl<'a> CodeGenerator<'a> {
    pub fn gen_while_expr(&mut self, expr: &WhileExprAst) -> Value<'a> {
        let initial_block = self.builder.get_insert_block().unwrap();
        let cond_block = self.context.insert_basic_block_after(initial_block, "");
        let body_block = self.context.insert_basic_block_after(cond_block, "");
        let end_block = self.context.insert_basic_block_after(body_block, "");

        self.builder.build_unconditional_branch(cond_block);
        self.builder.position_at_end(cond_block);

        let cond_value = self
            .gen_rvalue_expr(&expr.block.cond)
            .unwrap()
            .into_int_value();

        let cond_value =
            self.builder
                .build_int_compare(IntPredicate::EQ, cond_value, self.llvm_true, "");

        self.builder
            .build_conditional_branch(cond_value, body_block, end_block);

        self.builder.position_at_end(body_block);
        self.gen_block_expr(&expr.block.expr);
        self.builder.build_unconditional_branch(cond_block);

        self.builder.position_at_end(end_block);
        Value::Void
    }
}