use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::AssignStmtAst;
use cool_parser::AssignOp;
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_assign_stmt(&mut self, assign: &AssignStmtAst) {
        let _ = self.gen_assign_stmt2(assign);
    }

    pub fn gen_assign_stmt2(&mut self, assign: &AssignStmtAst) -> Option<()> {
        let lhs = self.gen_expr(&assign.lhs, None);
        if self.builder.current_block_diverges() {
            return None;
        }

        let lhs_ty_id = assign.lhs.expr_id().ty_id;

        let gen_int_values = |gen: &mut Self| -> Option<_> {
            let lhs = gen
                .gen_loaded_value(lhs_ty_id, lhs)
                .unwrap()
                .into_int_value();

            let rhs = match gen.gen_loaded_expr(&assign.rhs) {
                Some(rhs) => rhs.into_int_value(),
                None => return None,
            };

            Some((lhs, rhs))
        };

        let gen_float_values = |gen: &mut Self| -> Option<_> {
            let lhs = gen
                .gen_loaded_value(lhs_ty_id, lhs)
                .unwrap()
                .into_float_value();

            let rhs = match gen.gen_loaded_expr(&assign.rhs) {
                Some(rhs) => rhs.into_float_value(),
                None => return None,
            };

            Some((lhs, rhs))
        };

        let value = match assign.assign_op {
            AssignOp::Eq => self.gen_loaded_expr(&assign.rhs)?,
            AssignOp::Add => {
                if lhs_ty_id.is_int() {
                    let (lhs, rhs) = gen_int_values(self)?;
                    self.builder
                        .build_int_add(lhs, rhs, "")
                        .as_basic_value_enum()
                } else {
                    let (lhs, rhs) = gen_float_values(self)?;
                    self.builder
                        .build_float_add(lhs, rhs, "")
                        .as_basic_value_enum()
                }
            }
            AssignOp::Sub => {
                if lhs_ty_id.is_int() {
                    let (lhs, rhs) = gen_int_values(self)?;
                    self.builder
                        .build_int_sub(lhs, rhs, "")
                        .as_basic_value_enum()
                } else {
                    let (lhs, rhs) = gen_float_values(self)?;
                    self.builder
                        .build_float_sub(lhs, rhs, "")
                        .as_basic_value_enum()
                }
            }
            AssignOp::Mul => {
                if lhs_ty_id.is_int() {
                    let (lhs, rhs) = gen_int_values(self)?;
                    self.builder
                        .build_int_mul(lhs, rhs, "")
                        .as_basic_value_enum()
                } else {
                    let (lhs, rhs) = gen_float_values(self)?;
                    self.builder
                        .build_float_mul(lhs, rhs, "")
                        .as_basic_value_enum()
                }
            }
            AssignOp::Div => {
                if lhs_ty_id.is_signed_int() {
                    let (lhs, rhs) = gen_int_values(self)?;
                    self.builder
                        .build_int_signed_div(lhs, rhs, "")
                        .as_basic_value_enum()
                } else if lhs_ty_id.is_unsigned_int() {
                    let (lhs, rhs) = gen_int_values(self)?;
                    self.builder
                        .build_int_unsigned_div(lhs, rhs, "")
                        .as_basic_value_enum()
                } else {
                    let (lhs, rhs) = gen_float_values(self)?;
                    self.builder
                        .build_float_div(lhs, rhs, "")
                        .as_basic_value_enum()
                }
            }
            AssignOp::Rem => {
                if lhs_ty_id.is_signed_int() {
                    let (lhs, rhs) = gen_int_values(self)?;
                    self.builder
                        .build_int_signed_rem(lhs, rhs, "")
                        .as_basic_value_enum()
                } else if lhs_ty_id.is_unsigned_int() {
                    let (lhs, rhs) = gen_int_values(self)?;
                    self.builder
                        .build_int_unsigned_rem(lhs, rhs, "")
                        .as_basic_value_enum()
                } else {
                    let (lhs, rhs) = gen_float_values(self)?;
                    self.builder
                        .build_float_rem(lhs, rhs, "")
                        .as_basic_value_enum()
                }
            }
            AssignOp::Or => {
                let (lhs, rhs) = gen_int_values(self)?;
                self.builder.build_or(lhs, rhs, "").as_basic_value_enum()
            }
            AssignOp::And => {
                let (lhs, rhs) = gen_int_values(self)?;
                self.builder.build_and(lhs, rhs, "").as_basic_value_enum()
            }
            AssignOp::Xor => {
                let (lhs, rhs) = gen_int_values(self)?;
                self.builder.build_xor(lhs, rhs, "").as_basic_value_enum()
            }
            AssignOp::Shl => {
                let (lhs, rhs) = gen_int_values(self)?;
                self.builder
                    .build_left_shift(lhs, rhs, "")
                    .as_basic_value_enum()
            }
            AssignOp::Shr => {
                let (lhs, rhs) = gen_int_values(self)?;
                let sign_extend = lhs_ty_id.is_signed_int();

                self.builder
                    .build_right_shift(lhs, rhs, sign_extend, "")
                    .as_basic_value_enum()
            }
        };

        if let Value::Memory(memory) = lhs {
            self.builder.build_store(memory, value);
        }

        None
    }
}
