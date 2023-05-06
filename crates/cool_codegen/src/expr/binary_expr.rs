use crate::CodeGenerator;
use cool_ast::{
    ArithmeticBinOpAst, BinOpAst, BinaryExprAst, BitwiseBinOpAst, ComparisonBinOpAst, ExprAst,
};
use inkwell::values::{AnyValue, AnyValueEnum, IntValue};
use inkwell::{FloatPredicate as FloatP, IntPredicate as IntP};

impl<'a> CodeGenerator<'a> {
    pub fn gen_binary_expr(&mut self, expr: &BinaryExprAst) -> AnyValueEnum<'a> {
        let lhs = &expr.lhs;
        let rhs = &expr.rhs;

        match expr.bin_op {
            BinOpAst::Arithmetic(bin_op) => self.gen_arithmetic_expr(lhs, rhs, bin_op),
            BinOpAst::Comparison(bin_op) => self.gen_comparison_expr(lhs, rhs, bin_op),
            BinOpAst::Bitwise(bin_op) => self.gen_bitwise_expr(lhs, rhs, bin_op),
            _ => todo!(),
        }
    }

    fn gen_arithmetic_expr(
        &mut self,
        lhs: &ExprAst,
        rhs: &ExprAst,
        bin_op: ArithmeticBinOpAst,
    ) -> AnyValueEnum<'a> {
        let lhs = {
            let value = self.gen_expr(lhs);
            self.gen_loaded_value(value).unwrap()
        };
        let rhs = {
            let value = self.gen_expr(rhs);
            self.gen_loaded_value(value).unwrap()
        };

        match bin_op {
            ArithmeticBinOpAst::IntAdd => {
                self.builder
                    .build_int_add(lhs.into_int_value(), rhs.into_int_value(), "")
                    .as_any_value_enum()
            }
            ArithmeticBinOpAst::IntSub => {
                self.builder
                    .build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "")
                    .as_any_value_enum()
            }
            ArithmeticBinOpAst::IntMul => {
                self.builder
                    .build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "")
                    .as_any_value_enum()
            }
            ArithmeticBinOpAst::UintDiv => {
                self.builder
                    .build_int_unsigned_div(lhs.into_int_value(), rhs.into_int_value(), "")
                    .as_any_value_enum()
            }
            ArithmeticBinOpAst::SintDiv => {
                self.builder
                    .build_int_signed_div(lhs.into_int_value(), rhs.into_int_value(), "")
                    .as_any_value_enum()
            }
            ArithmeticBinOpAst::UintRem => {
                self.builder
                    .build_int_unsigned_rem(lhs.into_int_value(), rhs.into_int_value(), "")
                    .as_any_value_enum()
            }
            ArithmeticBinOpAst::SintRem => {
                self.builder
                    .build_int_signed_rem(lhs.into_int_value(), rhs.into_int_value(), "")
                    .as_any_value_enum()
            }
            ArithmeticBinOpAst::FloatAdd => {
                self.builder
                    .build_float_add(lhs.into_float_value(), rhs.into_float_value(), "")
                    .as_any_value_enum()
            }
            ArithmeticBinOpAst::FloatSub => {
                self.builder
                    .build_float_sub(lhs.into_float_value(), rhs.into_float_value(), "")
                    .as_any_value_enum()
            }
            ArithmeticBinOpAst::FloatMul => {
                self.builder
                    .build_float_mul(lhs.into_float_value(), rhs.into_float_value(), "")
                    .as_any_value_enum()
            }
            ArithmeticBinOpAst::FloatDiv => {
                self.builder
                    .build_float_div(lhs.into_float_value(), rhs.into_float_value(), "")
                    .as_any_value_enum()
            }
            ArithmeticBinOpAst::FloatRem => {
                self.builder
                    .build_float_rem(lhs.into_float_value(), rhs.into_float_value(), "")
                    .as_any_value_enum()
            }
        }
    }

    fn gen_comparison_expr(
        &mut self,
        lhs: &ExprAst,
        rhs: &ExprAst,
        bin_op: ComparisonBinOpAst,
    ) -> AnyValueEnum<'a> {
        let lhs = {
            let value = self.gen_expr(lhs);
            self.gen_loaded_value(value).unwrap()
        };
        let rhs = {
            let value = self.gen_expr(rhs);
            self.gen_loaded_value(value).unwrap()
        };

        let value = match bin_op {
            ComparisonBinOpAst::IntEq => self.util_gen_int_compare(lhs, rhs, IntP::EQ),
            ComparisonBinOpAst::IntNe => self.util_gen_int_compare(lhs, rhs, IntP::NE),
            ComparisonBinOpAst::UintLt => self.util_gen_int_compare(lhs, rhs, IntP::ULT),
            ComparisonBinOpAst::SintLt => self.util_gen_int_compare(lhs, rhs, IntP::SLT),
            ComparisonBinOpAst::UintLe => self.util_gen_int_compare(lhs, rhs, IntP::ULE),
            ComparisonBinOpAst::SintLe => self.util_gen_int_compare(lhs, rhs, IntP::SLE),
            ComparisonBinOpAst::UintGt => self.util_gen_int_compare(lhs, rhs, IntP::UGT),
            ComparisonBinOpAst::SintGt => self.util_gen_int_compare(lhs, rhs, IntP::SGT),
            ComparisonBinOpAst::UintGe => self.util_gen_int_compare(lhs, rhs, IntP::SGE),
            ComparisonBinOpAst::SintGe => self.util_gen_int_compare(lhs, rhs, IntP::SGE),
            ComparisonBinOpAst::FloatEq => self.util_gen_float_compare(lhs, rhs, FloatP::OEQ),
            ComparisonBinOpAst::FloatNe => self.util_gen_float_compare(lhs, rhs, FloatP::ONE),
            ComparisonBinOpAst::FloatLt => self.util_gen_float_compare(lhs, rhs, FloatP::OLT),
            ComparisonBinOpAst::FloatLe => self.util_gen_float_compare(lhs, rhs, FloatP::OLE),
            ComparisonBinOpAst::FloatGt => self.util_gen_float_compare(lhs, rhs, FloatP::OGT),
            ComparisonBinOpAst::FloatGe => self.util_gen_float_compare(lhs, rhs, FloatP::OGE),
        };

        self.builder
            .build_int_z_extend(value, self.tys.i8_ty(), "")
            .as_any_value_enum()
    }

    fn gen_bitwise_expr(
        &mut self,
        lhs: &ExprAst,
        rhs: &ExprAst,
        bin_op: BitwiseBinOpAst,
    ) -> AnyValueEnum<'a> {
        let lhs = {
            let value = self.gen_expr(lhs);
            self.gen_loaded_value(value).unwrap().into_int_value()
        };
        let rhs = {
            let value = self.gen_expr(rhs);
            self.gen_loaded_value(value).unwrap().into_int_value()
        };

        let value = match bin_op {
            BitwiseBinOpAst::And => self.builder.build_and(lhs, rhs, ""),
            BitwiseBinOpAst::Or => self.builder.build_or(lhs, rhs, ""),
            BitwiseBinOpAst::Xor => self.builder.build_xor(lhs, rhs, ""),
            BitwiseBinOpAst::Shl => self.builder.build_left_shift(lhs, rhs, ""),
            BitwiseBinOpAst::Shr => self.builder.build_right_shift(lhs, rhs, false, ""),
            BitwiseBinOpAst::SignExtendShr => self.builder.build_right_shift(lhs, rhs, true, ""),
        };

        value.as_any_value_enum()
    }

    fn util_gen_int_compare(
        &mut self,
        lhs: AnyValueEnum<'a>,
        rhs: AnyValueEnum<'a>,
        predicate: IntP,
    ) -> IntValue<'a> {
        self.builder
            .build_int_compare(predicate, lhs.into_int_value(), rhs.into_int_value(), "")
    }

    fn util_gen_float_compare(
        &mut self,
        lhs: AnyValueEnum<'a>,
        rhs: AnyValueEnum<'a>,
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
