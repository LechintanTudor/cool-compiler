use crate::{CodeGenerator, LoadedValue};
use cool_ast::CondExprAst;
use inkwell::IntPredicate;
use std::iter;

impl<'a> CodeGenerator<'a> {
    pub fn gen_cond_expr(&mut self, expr: &CondExprAst) -> LoadedValue<'a> {
        let start_block = self.builder.get_insert_block().unwrap();
        let end_block = self.context.insert_basic_block_after(start_block, "");

        let phi_value = expr
            .is_exhaustive()
            .then(|| {
                let ty_id = self.resolve[expr.expr_id].ty_id;

                self.tys[ty_id].map(|ty| {
                    self.builder.position_at_end(end_block);
                    self.builder.build_phi(ty, "")
                })
            })
            .flatten();

        let else_block = expr
            .is_exhaustive()
            .then(|| self.context.prepend_basic_block(end_block, ""));

        let fallback_block = else_block.unwrap_or(end_block);

        let mut cond_block_pairs = {
            let else_if_blocks = expr.else_if_blocks.iter().map(|block_ast| {
                let block = self.context.prepend_basic_block(fallback_block, "");
                (block_ast, block)
            });

            iter::once((Box::as_ref(&expr.if_block), start_block))
                .chain(else_if_blocks)
                .peekable()
        };

        while let Some((ast_block, block)) = cond_block_pairs.next() {
            let else_block = cond_block_pairs
                .peek()
                .map(|(_, block)| *block)
                .unwrap_or(fallback_block);

            let then_block = self.context.insert_basic_block_after(block, "");
            self.builder.position_at_end(then_block);
            let then_value = self.gen_block_expr(&ast_block.expr);
            self.builder.build_unconditional_branch(end_block);

            if let Some(phi_value) = phi_value {
                phi_value.add_incoming(&[(then_value.as_basic_value().unwrap(), then_block)]);
            }

            self.builder.position_at_end(block);

            let cond_value = self
                .gen_loaded_expr(&ast_block.cond)
                .into_basic_value()
                .into_int_value();

            let cond_value =
                self.builder
                    .build_int_compare(IntPredicate::EQ, cond_value, self.llvm_true, "");

            self.builder
                .build_conditional_branch(cond_value, then_block, else_block);
        }

        if let (Some(else_block_ast), Some(else_block)) = (&expr.else_block, else_block) {
            self.builder.position_at_end(else_block);
            let else_value = self.gen_block_expr(else_block_ast);
            self.builder.build_unconditional_branch(end_block);

            if let Some(phi_value) = phi_value {
                phi_value.add_incoming(&[(else_value.as_basic_value().unwrap(), else_block)]);
            }
        }

        self.builder.position_at_end(end_block);

        phi_value
            .map(|value| LoadedValue::Register(value.as_basic_value()))
            .unwrap_or(LoadedValue::Void)
    }
}
