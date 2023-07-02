use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::UnaryExprAst;
use cool_parser::UnaryOpKind;
use inkwell::values::BasicValue;
use inkwell::IntPredicate;

impl<'a> CodeGenerator<'a> {
    pub fn gen_unary_expr(&mut self, unary_expr: &UnaryExprAst) -> Value<'a> {
        let ty_id = unary_expr.expr_id.ty_id;

        match unary_expr.op.kind {
            UnaryOpKind::Minus => {
                let value = {
                    let value = self.gen_loaded_expr(&unary_expr.expr);
                    if self.builder.current_block_diverges() {
                        return Value::Void;
                    }

                    value.unwrap()
                };

                if ty_id.is_int() {
                    self.builder
                        .build_int_neg(value.into_int_value(), "")
                        .as_basic_value_enum()
                        .into()
                } else {
                    debug_assert!(ty_id.is_float());

                    self.builder
                        .build_float_neg(value.into_float_value(), "")
                        .as_basic_value_enum()
                        .into()
                }
            }
            UnaryOpKind::Not => {
                let value = {
                    let value = self.gen_loaded_expr(&unary_expr.expr);
                    if self.builder.current_block_diverges() {
                        return Value::Void;
                    }

                    value.unwrap().into_int_value()
                };

                if ty_id.is_int() {
                    self.builder
                        .build_not(value, "")
                        .as_basic_value_enum()
                        .into()
                } else {
                    let value =
                        self.builder
                            .build_int_compare(IntPredicate::EQ, value, self.llvm_true, "");

                    self.builder
                        .build_select(value, self.llvm_false, self.llvm_true, "")
                        .as_basic_value_enum()
                        .into()
                }
            }
            UnaryOpKind::Addr { .. } => {
                match self.gen_expr(&unary_expr.expr, None) {
                    Value::Void => {
                        let base_ty_id = unary_expr.expr.expr_id().ty_id;
                        let base_align = self.resolve.get_ty_def(base_ty_id).unwrap().align;
                        let ptr_value = self.tys.isize_ty().const_int(base_align, false);

                        self.builder
                            .build_int_to_ptr(ptr_value, self.tys.i8_ptr_ty(), "")
                            .as_basic_value_enum()
                            .into()
                    }
                    Value::Fn(fn_value) => fn_value.as_global_value().as_basic_value_enum().into(),
                    Value::Register(_) => {
                        todo!()
                    }
                    Value::Memory(memory) => memory.as_basic_value_enum().into(),
                }
            }
        }
    }
}
