use crate::CodeGenerator;
use cool_ast::{ArithmeticBinOpAst, BinOpAst, BinaryExprAst, ExprAst};
use inkwell::values::{AnyValue, AnyValueEnum};

impl<'a> CodeGenerator<'a> {
    pub fn gen_binary_expr(&mut self, expr: &BinaryExprAst) -> AnyValueEnum<'a> {
        match expr.bin_op {
            BinOpAst::Arithmetic(bin_op) => self.gen_arithmetic_expr(&expr.lhs, &expr.rhs, bin_op),
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
}
