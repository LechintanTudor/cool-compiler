use crate::{AnyTypeEnumExt, AnyValueEnumExt, CodeGenerator, Value};
use cool_ast::{CondBlockAst, CondExprAst};
use inkwell::basic_block::BasicBlock;
use inkwell::values::{AnyValue, BasicValue, PhiValue};
use inkwell::IntPredicate;

impl<'a> CodeGenerator<'a> {
    pub fn gen_cond_expr(&mut self, expr: &CondExprAst) -> Value<'a> {
        let initial_block = self.builder.get_insert_block().unwrap();

        let end_if_block = self
            .context
            .insert_basic_block_after(initial_block, "end_if");

        let expr_ty_id = self.resolve[expr.expr_id];

        let phi_value = if !self.resolve.is_ty_id_zst(expr_ty_id) {
            self.builder.position_at_end(end_if_block);
            let phi_ty = self.tys[expr_ty_id].into_basic_type();
            Some(self.builder.build_phi(phi_ty, ""))
        } else {
            None
        };

        let else_block = expr
            .else_block
            .as_ref()
            .map(|_| self.context.insert_basic_block_after(initial_block, "else"));

        let cond_block_pairs = {
            let mut cond_blocks = vec![(Box::as_ref(&expr.if_block), initial_block)];
            let mut current_block = initial_block;

            for cond_block_ast in expr.else_if_blocks.iter() {
                let else_if_block = self
                    .context
                    .insert_basic_block_after(current_block, "then_cond");

                cond_blocks.push((cond_block_ast, else_if_block));
                current_block = else_if_block;
            }

            cond_blocks
        };

        let cond_fallback_block = else_block.unwrap_or(end_if_block);
        let mut cond_block_pair_iter = cond_block_pairs.iter().peekable();

        while let Some((cond_block_ast, cond_block)) = cond_block_pair_iter.next() {
            let else_block = cond_block_pair_iter
                .peek()
                .map(|(_, block)| *block)
                .unwrap_or(cond_fallback_block);

            self.builder.position_at_end(*cond_block);
            self.util_gen_if_then_else_block(cond_block_ast, else_block, end_if_block, phi_value);
        }

        if let (Some(else_block_ast), Some(else_block)) = (expr.else_block.as_ref(), else_block) {
            self.builder.position_at_end(else_block);
            let value = self.gen_block_expr(&else_block_ast);

            if let Some(phi_value) = phi_value {
                let value = self.gen_loaded_value(value).unwrap().into_basic_value();
                phi_value.add_incoming(&[(&value as &dyn BasicValue, else_block)]);
            }

            self.builder.build_unconditional_branch(end_if_block);
        }

        self.builder.position_at_end(end_if_block);

        phi_value
            .map(|value| Value::Rvalue(value.as_basic_value().as_any_value_enum()))
            .unwrap_or(Value::Void)
    }

    fn util_gen_if_then_else_block(
        &mut self,
        cond_block_ast: &CondBlockAst,
        else_block: BasicBlock,
        end_if_block: BasicBlock,
        phi_value: Option<PhiValue>,
    ) {
        let current_block = self.builder.get_insert_block().unwrap();

        let then_block = self.context.insert_basic_block_after(current_block, "then");
        self.builder.position_at_end(then_block);
        let value = self.gen_block_expr(&cond_block_ast.expr);

        if let Some(phi_value) = phi_value {
            let value = self.gen_loaded_value(value).unwrap().into_basic_value();
            phi_value.add_incoming(&[(&value as &dyn BasicValue, then_block)]);
        }

        self.builder.build_unconditional_branch(end_if_block);

        let cond_value = self
            .gen_rvalue_expr(&cond_block_ast.cond)
            .unwrap()
            .into_int_value();

        let cond_value = self.builder.build_int_compare(
            IntPredicate::EQ,
            cond_value,
            self.llvm_true,
            "cond_expr",
        );

        self.builder.position_at_end(current_block);
        self.builder
            .build_conditional_branch(cond_value, then_block, else_block);
    }
}
