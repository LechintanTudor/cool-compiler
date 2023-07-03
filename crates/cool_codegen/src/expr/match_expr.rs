use crate::{BuilderExt, CodeGenerator, LoadedValue, Value};
use cool_ast::MatchExprAst;
use cool_lexer::sym;
use cool_resolve::TaggedUnionKind;
use inkwell::basic_block::BasicBlock;
use inkwell::values::{BasicValueEnum, IntValue};
use inkwell::IntPredicate;

impl<'a> CodeGenerator<'a> {
    pub fn gen_match_expr(&mut self, expr: &MatchExprAst) -> LoadedValue<'a> {
        let matched_expr_value = self.gen_expr(&expr.matched_expr, None);
        if self.builder.current_block_diverges() {
            return LoadedValue::None;
        }

        let matched_expr_ty_id = expr.matched_expr.expr_id().ty_id;
        let matched_expr_ty = self.tys[matched_expr_ty_id].unwrap();

        let matched_expr_ptr = match matched_expr_value {
            Value::Memory(memory) => memory,
            Value::Register(value) => self.util_gen_init(value),
            Value::Void | Value::Fn(_) => panic!("invalid value for matched expression"),
        };

        let tagged_union_kind = self
            .resolve
            .get_ty_def(matched_expr_ty_id)
            .unwrap()
            .kind
            .as_tagged_union()
            .unwrap()
            .kind;

        let index_value = match tagged_union_kind {
            TaggedUnionKind::Basic { .. } => {
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
            }
            TaggedUnionKind::NullablePtr => {
                let value = self
                    .builder
                    .build_load(self.tys.isize_ty(), matched_expr_ptr, "")
                    .into_int_value();

                let zero_isize = self.tys.isize_ty().const_zero();

                let select_value =
                    self.builder
                        .build_int_compare(IntPredicate::NE, value, zero_isize, "");

                self.builder
                    .build_select(
                        select_value,
                        self.tys.i8_ty().const_zero(),
                        self.tys.i8_ty().const_int(1, false),
                        "",
                    )
                    .into_int_value()
            }
        };

        let switch_block = self.builder.current_block();
        let else_block = self.append_block_after_current_block();
        let end_block = self.append_block_after(else_block);

        let mut arm_blocks = Vec::<(IntValue<'a>, BasicBlock<'a>)>::new();
        let mut phi_values = Vec::<(BasicBlock, BasicValueEnum<'a>)>::new();

        for arm in expr.arms.iter() {
            let block = self.append_block_after_current_block();
            self.builder.position_at_end(block);

            if let Some(binding_id) = arm.binding_id {
                let binding_ty_id = self.resolve[binding_id].ty_id;
                let binding_value = match self.tys[binding_ty_id] {
                    Some(binding_ty) => {
                        let value = self.builder.build_load(binding_ty, matched_expr_ptr, "");
                        Value::Memory(self.util_gen_init(value))
                    }
                    None => Value::Void,
                };

                self.bindings.insert(binding_id, binding_value);
            }

            let value = self.gen_loaded_expr(&arm.expr);

            if !self.builder.current_block_diverges() {
                if let Some(value) = value {
                    phi_values.push((self.builder.current_block(), value));
                }

                self.builder.build_unconditional_branch(end_block);
            }

            let arm_index = match tagged_union_kind {
                TaggedUnionKind::Basic { .. } => {
                    expr.get_variant_index(arm.arm_ty_id)
                        .map(|i| self.tys.i8_ty().const_int(i as _, false))
                        .unwrap()
                }

                TaggedUnionKind::NullablePtr => {
                    let value = if arm.arm_ty_id.get_value().is_ptr_like() {
                        0
                    } else {
                        1
                    };

                    self.tys.i8_ty().const_int(value, false)
                }
            };

            arm_blocks.push((arm_index, block));
        }

        self.builder.position_at_end(else_block);

        match expr.else_arm.as_ref() {
            Some(else_arm) => {
                let value = self.gen_loaded_expr(else_arm);

                if !self.builder.current_block_diverges() {
                    if let Some(value) = value {
                        phi_values.push((self.builder.current_block(), value));
                    }

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
                    phi_value.add_incoming(&[(&value, block)]);
                }

                phi_value.as_basic_value().into()
            }
            _ => LoadedValue::None,
        }
    }
}
