use crate::{BuilderExt, CodeGenerator, LoadedValue};
use cool_ast::CondExprAst;
use inkwell::basic_block::BasicBlock;

impl<'a> CodeGenerator<'a> {
    pub fn gen_cond_expr(&mut self, expr: &CondExprAst) -> LoadedValue<'a> {
        let end_block = self.append_block_after_current_block();
        let mut else_expr = expr.else_block.as_ref().map(Box::as_ref);
        let mut phi_values = Vec::<(BasicBlock, LoadedValue)>::new();

        for (i, cond_block) in expr.cond_blocks.iter().enumerate() {
            // Condition
            let cond_value = self.gen_loaded_expr(&cond_block.cond);

            if self.builder.current_block_diverges() {
                if i == 0 {
                    end_block.remove_from_function().unwrap();
                    return LoadedValue::Void;
                }

                else_expr = None;
                break;
            }

            let cond_value = cond_value.into_basic_value().into_int_value();
            let bool_cond_value = self.builder.build_bool(cond_value);

            let body_block = self.append_block_after_current_block();
            let next_block = if i + 1 == expr.cond_blocks.len() && else_expr.is_none() {
                end_block
            } else {
                self.append_block_after(body_block)
            };

            self.builder
                .build_conditional_branch(bool_cond_value, body_block, next_block);

            // Body
            self.builder.position_at_end(body_block);
            let body_value = self.gen_block_expr(&cond_block.expr);

            if !self.builder.current_block_diverges() {
                phi_values.push((self.builder.current_block(), body_value));
                self.builder.build_unconditional_branch(end_block);
            }

            self.builder.position_at_end(next_block);
        }

        if let Some(else_expr) = else_expr {
            let else_value = self.gen_block_expr(else_expr);

            if !self.builder.current_block_diverges() {
                phi_values.push((self.builder.current_block(), else_value));
                self.builder.build_unconditional_branch(end_block);
            }
        }

        self.builder.position_at_end(end_block);

        let expr_ty_id = self.resolve[expr.expr_id].ty_id;
        if expr_ty_id.def.is_zero_sized() || phi_values.is_empty() {
            return LoadedValue::Void;
        }

        let expr_ty = self.tys[expr_ty_id].unwrap();
        let phi_value = self.builder.build_phi(expr_ty, "");

        for (block, value) in phi_values {
            phi_value.add_incoming(&[(&value.into_basic_value(), block)]);
        }

        phi_value.as_basic_value().into()
    }
}
