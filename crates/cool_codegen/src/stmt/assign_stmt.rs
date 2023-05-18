use crate::{CodeGenerator, Value};
use cool_ast::AssignStmtAst;
use cool_parser::AssignOp;
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_assign_stmt(&mut self, assign: &AssignStmtAst) {
        let lhs = self.gen_expr(&assign.lhs);
        let lhs_ty_id = self.resolve[assign.lhs.expr_id()].ty_id;

        if self.resolve.is_ty_id_zst(lhs_ty_id) {
            self.gen_expr(&assign.rhs);
            return;
        }

        let Value::Memory(value) = lhs else {
            panic!("assignment lhs is not an lvalue");
        };

        let gen_int_values = |gen: &mut Self| {
            let lhs = gen
                .gen_loaded_value(lhs)
                .into_basic_value()
                .into_int_value();

            let rhs = gen
                .gen_loaded_expr(&assign.rhs)
                .into_basic_value()
                .into_int_value();

            (lhs, rhs)
        };

        let gen_float_values = |gen: &mut Self| {
            let lhs = gen
                .gen_loaded_value(lhs)
                .into_basic_value()
                .into_float_value();

            let rhs = gen
                .gen_loaded_expr(&assign.rhs)
                .into_basic_value()
                .into_float_value();

            (lhs, rhs)
        };

        let result_value = match assign.assign_op {
            AssignOp::Eq => self.gen_loaded_expr(&assign.rhs).into_basic_value(),
            AssignOp::Add => {
                if lhs_ty_id.is_int() {
                    let (lhs, rhs) = gen_int_values(self);
                    self.builder
                        .build_int_add(lhs, rhs, "")
                        .as_basic_value_enum()
                } else {
                    let (lhs, rhs) = gen_float_values(self);
                    self.builder
                        .build_float_add(lhs, rhs, "")
                        .as_basic_value_enum()
                }
            }
            AssignOp::Sub => {
                if lhs_ty_id.is_int() {
                    let (lhs, rhs) = gen_int_values(self);
                    self.builder
                        .build_int_sub(lhs, rhs, "")
                        .as_basic_value_enum()
                } else {
                    let (lhs, rhs) = gen_float_values(self);
                    self.builder
                        .build_float_sub(lhs, rhs, "")
                        .as_basic_value_enum()
                }
            }
            AssignOp::Mul => {
                if lhs_ty_id.is_int() {
                    let (lhs, rhs) = gen_int_values(self);
                    self.builder
                        .build_int_mul(lhs, rhs, "")
                        .as_basic_value_enum()
                } else {
                    let (lhs, rhs) = gen_float_values(self);
                    self.builder
                        .build_float_mul(lhs, rhs, "")
                        .as_basic_value_enum()
                }
            }
            AssignOp::Div => {
                if lhs_ty_id.is_signed_int() {
                    let (lhs, rhs) = gen_int_values(self);
                    self.builder
                        .build_int_signed_div(lhs, rhs, "")
                        .as_basic_value_enum()
                } else if lhs_ty_id.is_unsigned_int() {
                    let (lhs, rhs) = gen_int_values(self);
                    self.builder
                        .build_int_unsigned_div(lhs, rhs, "")
                        .as_basic_value_enum()
                } else {
                    let (lhs, rhs) = gen_float_values(self);
                    self.builder
                        .build_float_div(lhs, rhs, "")
                        .as_basic_value_enum()
                }
            }
            AssignOp::Rem => {
                if lhs_ty_id.is_signed_int() {
                    let (lhs, rhs) = gen_int_values(self);
                    self.builder
                        .build_int_signed_rem(lhs, rhs, "")
                        .as_basic_value_enum()
                } else if lhs_ty_id.is_unsigned_int() {
                    let (lhs, rhs) = gen_int_values(self);
                    self.builder
                        .build_int_unsigned_rem(lhs, rhs, "")
                        .as_basic_value_enum()
                } else {
                    let (lhs, rhs) = gen_float_values(self);
                    self.builder
                        .build_float_rem(lhs, rhs, "")
                        .as_basic_value_enum()
                }
            }
            AssignOp::Or => {
                let (lhs, rhs) = gen_int_values(self);
                self.builder.build_or(lhs, rhs, "").as_basic_value_enum()
            }
            AssignOp::And => {
                let (lhs, rhs) = gen_int_values(self);
                self.builder.build_and(lhs, rhs, "").as_basic_value_enum()
            }
            AssignOp::Xor => {
                let (lhs, rhs) = gen_int_values(self);
                self.builder.build_xor(lhs, rhs, "").as_basic_value_enum()
            }
            AssignOp::Shl => {
                let (lhs, rhs) = gen_int_values(self);
                self.builder
                    .build_left_shift(lhs, rhs, "")
                    .as_basic_value_enum()
            }
            AssignOp::Shr => {
                let (lhs, rhs) = gen_int_values(self);
                let sign_extend = lhs_ty_id.is_signed_int();

                self.builder
                    .build_right_shift(lhs, rhs, sign_extend, "")
                    .as_basic_value_enum()
            }
        };

        self.builder.build_store(value.pointer, result_value);
    }
}
