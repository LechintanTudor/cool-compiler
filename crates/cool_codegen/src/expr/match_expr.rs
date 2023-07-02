use crate::{BuilderExt, CodeGenerator, LoadedValue, Value};
use cool_ast::MatchExprAst;
use cool_lexer::sym;
use inkwell::basic_block::BasicBlock;
use inkwell::values::IntValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_match_expr(&mut self, expr: &MatchExprAst) -> LoadedValue<'a> {
        let matched_expr_value = self.gen_expr(&expr.matched_expr, None);
        if self.builder.current_block_diverges() {
            return LoadedValue::None;
        }

        let matched_expr_ty_id = expr.matched_expr.expr_id().ty_id;
        let matched_expr_ty = self.tys[matched_expr_ty_id].unwrap();

        let matched_expr_ptr = match matched_expr_value {
            Value::Memory(memory) => memory.ptr,
            Value::Register(value) => self.util_gen_init(value),
            Value::Void | Value::Fn(_) => unreachable!(),
        };

        let index_value = {
            let index_field_index = self
                .tys
                .get_field_map(matched_expr_ty_id)
                .get(sym::VARIANT_INDEX)
                .unwrap();

            let index_field_ptr = self
                .builder
                .build_struct_gep(matched_expr_ty, matched_expr_ptr, index_field_index, "")
                .unwrap();

            self.builder
                .build_load(self.tys.i8_ty(), index_field_ptr, "")
                .into_int_value()
        };

        let switch_block = self.builder.current_block();
        let else_block = self.append_block_after_current_block();
        let end_block = self.append_block_after(else_block);

        let mut arm_blocks = Vec::<(IntValue<'a>, BasicBlock<'a>)>::new();
        let mut phi_values = Vec::<(BasicBlock, LoadedValue)>::new();

        for arm in expr.arms.iter() {
            let block = self.append_block_after_current_block();
            self.builder.position_at_end(block);

            if let Some(binding_id) = arm.binding_id {
                let bindind_ty = self.tys[self.resolve[binding_id].ty_id];

                let binding_value = bindind_ty
                    .map(|ty| self.builder.build_load(ty, matched_expr_ptr, ""))
                    .map(Value::Register)
                    .unwrap_or(Value::Void);

                self.bindings.insert(binding_id, binding_value);
            }

            let value = self.gen_loaded_expr(&arm.expr);

            if !self.builder.current_block_diverges() {
                phi_values.push((self.builder.current_block(), value));
                self.builder.build_unconditional_branch(end_block);
            }

            let arm_index = expr
                .get_variant_index(arm.arm_ty_id)
                .map(|i| self.tys.i8_ty().const_int(i as _, false))
                .unwrap();

            arm_blocks.push((arm_index, block));
        }

        self.builder.position_at_end(else_block);

        match expr.else_arm.as_ref() {
            Some(else_arm) => {
                let value = self.gen_loaded_expr(else_arm);

                if !self.builder.current_block_diverges() {
                    phi_values.push((self.builder.current_block(), value));
                    self.builder.build_unconditional_branch(end_block);
                }
            }
            None => {
                self.builder.build_unreachable();
            }
        }

        self.builder.position_at_end(switch_block);
        self.builder
            .build_switch(index_value, else_block, &arm_blocks);

        self.builder.position_at_end(end_block);

        match self.tys[expr.expr_id.ty_id] {
            Some(ty) if !phi_values.is_empty() => {
                let phi_value = self.builder.build_phi(ty, "");

                for (block, value) in phi_values {
                    phi_value.add_incoming(&[(&value.into_basic_value(), block)]);
                }

                phi_value.as_basic_value().into()
            }
            _ => LoadedValue::None,
        }
    }
}
