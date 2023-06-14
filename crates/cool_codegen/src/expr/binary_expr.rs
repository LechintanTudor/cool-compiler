use crate::{BuilderExt, CodeGenerator};
use cool_ast::{BinaryExprAst, ExprAst};
use cool_parser::{ArithmeticOp, BinOp, BitwiseOp, ComparisonOp, LogicalOp};
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};
use inkwell::{FloatPredicate as FloatP, IntPredicate as IntP};

impl<'a> CodeGenerator<'a> {
    pub fn gen_binary_expr(&mut self, expr: &BinaryExprAst) -> BasicValueEnum<'a> {
        let lhs = &expr.lhs;
        let rhs = &expr.rhs;

        match expr.bin_op {
            BinOp::Arithmetic(op) => self.gen_arithmetic_expr(lhs, rhs, op),
            BinOp::Comparison(op) => self.gen_comparison_expr(lhs, rhs, op),
            BinOp::Bitwise(op) => self.gen_bitwise_expr(lhs, rhs, op),
            BinOp::Logical(op) => self.gen_logical_expr(lhs, rhs, op),
        }
    }

    fn gen_arithmetic_expr(
        &mut self,
        lhs: &ExprAst,
        rhs: &ExprAst,
        arithmetic_op: ArithmeticOp,
    ) -> BasicValueEnum<'a> {
        let ty_id = self.resolve[lhs.expr_id()].ty_id;
        let lhs = self.gen_loaded_expr(lhs).into_basic_value();
        let rhs = self.gen_loaded_expr(rhs).into_basic_value();

        match arithmetic_op {
            ArithmeticOp::Add => {
                if ty_id.is_int() {
                    self.builder
                        .build_int_add(lhs.into_int_value(), rhs.into_int_value(), "")
                        .as_basic_value_enum()
                } else {
                    self.builder
                        .build_float_add(lhs.into_float_value(), rhs.into_float_value(), "")
                        .as_basic_value_enum()
                }
            }
            ArithmeticOp::Sub => {
                if ty_id.is_int() {
                    self.builder
                        .build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "")
                        .as_basic_value_enum()
                } else {
                    self.builder
                        .build_float_sub(lhs.into_float_value(), rhs.into_float_value(), "")
                        .as_basic_value_enum()
                }
            }
            ArithmeticOp::Mul => {
                if ty_id.is_int() {
                    self.builder
                        .build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "")
                        .as_basic_value_enum()
                } else {
                    self.builder
                        .build_float_mul(lhs.into_float_value(), rhs.into_float_value(), "")
                        .as_basic_value_enum()
                }
            }
            ArithmeticOp::Div => {
                if ty_id.is_signed_int() {
                    self.builder
                        .build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "")
                        .as_basic_value_enum()
                } else if ty_id.is_unsigned_int() {
                    self.builder
                        .build_int_unsigned_div(lhs.into_int_value(), rhs.into_int_value(), "")
                        .as_basic_value_enum()
                } else {
                    self.builder
                        .build_float_div(lhs.into_float_value(), rhs.into_float_value(), "")
                        .as_basic_value_enum()
                }
            }
            ArithmeticOp::Rem => {
                if ty_id.is_signed_int() {
                    self.builder
                        .build_int_signed_rem(lhs.into_int_value(), rhs.into_int_value(), "")
                        .as_basic_value_enum()
                } else if ty_id.is_unsigned_int() {
                    self.builder
                        .build_int_unsigned_rem(lhs.into_int_value(), rhs.into_int_value(), "")
                        .as_basic_value_enum()
                } else {
                    self.builder
                        .build_float_rem(lhs.into_float_value(), rhs.into_float_value(), "")
                        .as_basic_value_enum()
                }
            }
        }
    }

    fn gen_comparison_expr(
        &mut self,
        lhs: &ExprAst,
        rhs: &ExprAst,
        comparison_op: ComparisonOp,
    ) -> BasicValueEnum<'a> {
        let ty_id = self.resolve[lhs.expr_id()].ty_id;

        let lhs = {
            match self.gen_loaded_expr(lhs).into_basic_value() {
                BasicValueEnum::PointerValue(ptr) => {
                    self.builder
                        .build_ptr_to_int(ptr, self.tys.isize_ty(), "")
                        .as_basic_value_enum()
                }
                value => value,
            }
        };
        let rhs = {
            match self.gen_loaded_expr(rhs).into_basic_value() {
                BasicValueEnum::PointerValue(ptr) => {
                    self.builder
                        .build_ptr_to_int(ptr, self.tys.isize_ty(), "")
                        .as_basic_value_enum()
                }
                value => value,
            }
        };

        let value = match comparison_op {
            ComparisonOp::Eq => {
                if ty_id.is_float() {
                    self.util_gen_float_compare(lhs, rhs, FloatP::OEQ)
                } else {
                    self.util_gen_int_compare(lhs, rhs, IntP::EQ)
                }
            }
            ComparisonOp::Ne => {
                if ty_id.is_float() {
                    self.util_gen_float_compare(lhs, rhs, FloatP::ONE)
                } else {
                    self.util_gen_int_compare(lhs, rhs, IntP::NE)
                }
            }
            ComparisonOp::Lt => {
                if ty_id.is_float() {
                    self.util_gen_float_compare(lhs, rhs, FloatP::OLE)
                } else if ty_id.is_signed_int() {
                    self.util_gen_int_compare(lhs, rhs, IntP::SLT)
                } else {
                    self.util_gen_int_compare(lhs, rhs, IntP::ULT)
                }
            }
            ComparisonOp::Le => {
                if ty_id.is_float() {
                    self.util_gen_float_compare(lhs, rhs, FloatP::OLE)
                } else if ty_id.is_signed_int() {
                    self.util_gen_int_compare(lhs, rhs, IntP::SLT)
                } else {
                    self.util_gen_int_compare(lhs, rhs, IntP::ULT)
                }
            }
            ComparisonOp::Gt => {
                if ty_id.is_float() {
                    self.util_gen_float_compare(lhs, rhs, FloatP::OGT)
                } else if ty_id.is_signed_int() {
                    self.util_gen_int_compare(lhs, rhs, IntP::SGT)
                } else {
                    self.util_gen_int_compare(lhs, rhs, IntP::UGT)
                }
            }
            ComparisonOp::Ge => {
                if ty_id.is_float() {
                    self.util_gen_float_compare(lhs, rhs, FloatP::OGE)
                } else if ty_id.is_signed_int() {
                    self.util_gen_int_compare(lhs, rhs, IntP::SGE)
                } else {
                    self.util_gen_int_compare(lhs, rhs, IntP::UGE)
                }
            }
        };

        self.builder
            .build_int_z_extend(value, self.tys.i8_ty(), "")
            .as_basic_value_enum()
    }

    fn gen_bitwise_expr(
        &mut self,
        lhs: &ExprAst,
        rhs: &ExprAst,
        bitwise_op: BitwiseOp,
    ) -> BasicValueEnum<'a> {
        let ty_id = self.resolve[lhs.expr_id()].ty_id;

        let lhs = self
            .gen_loaded_expr(lhs)
            .into_basic_value()
            .into_int_value();

        let rhs = self
            .gen_loaded_expr(rhs)
            .into_basic_value()
            .into_int_value();

        let value = match bitwise_op {
            BitwiseOp::And => self.builder.build_and(lhs, rhs, ""),
            BitwiseOp::Or => self.builder.build_or(lhs, rhs, ""),
            BitwiseOp::Xor => self.builder.build_xor(lhs, rhs, ""),
            BitwiseOp::Shl => self.builder.build_left_shift(lhs, rhs, ""),
            BitwiseOp::Shr => {
                let sign_extend = ty_id.is_signed_int();
                self.builder.build_right_shift(lhs, rhs, sign_extend, "")
            }
        };

        value.as_basic_value_enum()
    }

    fn gen_logical_expr(
        &mut self,
        lhs: &ExprAst,
        rhs: &ExprAst,
        logical_op: LogicalOp,
    ) -> BasicValueEnum<'a> {
        match logical_op {
            LogicalOp::And => self.gen_logical_and(lhs, rhs),
            LogicalOp::Or => self.gen_logical_or(lhs, rhs),
        }
    }

    fn gen_logical_and(&mut self, lhs: &ExprAst, rhs: &ExprAst) -> BasicValueEnum<'a> {
        let lhs_value = self
            .gen_loaded_expr(lhs)
            .into_basic_value()
            .into_int_value();
        let lhs_cond_value = self.builder.build_bool(lhs_value);

        let lhs_block = self.builder.get_insert_block().unwrap();
        let rhs_block = self.context.insert_basic_block_after(lhs_block, "");
        let end_block = self.context.insert_basic_block_after(rhs_block, "");

        self.builder
            .build_conditional_branch(lhs_cond_value, rhs_block, end_block);

        self.builder.position_at_end(rhs_block);
        let rhs_value = self
            .gen_loaded_expr(rhs)
            .into_basic_value()
            .into_int_value();
        self.builder.build_unconditional_branch(end_block);

        self.builder.position_at_end(end_block);
        let phi_value = self.builder.build_phi(self.tys.i8_ty(), "");
        phi_value.add_incoming(&[(&self.llvm_false, lhs_block)]);
        phi_value.add_incoming(&[(&rhs_value, rhs_block)]);
        phi_value.as_basic_value().as_basic_value_enum()
    }

    fn gen_logical_or(&mut self, lhs: &ExprAst, rhs: &ExprAst) -> BasicValueEnum<'a> {
        let lhs_value = self
            .gen_loaded_expr(lhs)
            .into_basic_value()
            .into_int_value();
        let lhs_cond_value = self.builder.build_bool(lhs_value);

        let lhs_block = self.builder.get_insert_block().unwrap();
        let rhs_block = self.context.insert_basic_block_after(lhs_block, "");
        let end_block = self.context.insert_basic_block_after(rhs_block, "");

        self.builder
            .build_conditional_branch(lhs_cond_value, end_block, rhs_block);

        self.builder.position_at_end(rhs_block);
        let rhs_value = self
            .gen_loaded_expr(rhs)
            .into_basic_value()
            .into_int_value();
        self.builder.build_unconditional_branch(end_block);

        self.builder.position_at_end(end_block);

        let phi_value = self.builder.build_phi(self.tys.i8_ty(), "");
        phi_value.add_incoming(&[(&self.llvm_true, lhs_block)]);
        phi_value.add_incoming(&([(&rhs_value, rhs_block)]));
        phi_value.as_basic_value().as_basic_value_enum()
    }

    fn util_gen_int_compare(
        &mut self,
        lhs: BasicValueEnum<'a>,
        rhs: BasicValueEnum<'a>,
        predicate: IntP,
    ) -> IntValue<'a> {
        self.builder
            .build_int_compare(predicate, lhs.into_int_value(), rhs.into_int_value(), "")
    }

    fn util_gen_float_compare(
        &mut self,
        lhs: BasicValueEnum<'a>,
        rhs: BasicValueEnum<'a>,
        predicate: FloatP,
    ) -> IntValue<'a> {
        self.builder.build_float_compare(
            predicate,
            lhs.into_float_value(),
            rhs.into_float_value(),
            "",
        )
    }
}
