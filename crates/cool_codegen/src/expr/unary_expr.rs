use crate::{CodeGenerator, Value};
use cool_ast::UnaryExprAst;
use cool_parser::UnaryOpKind;
use cool_resolve::tys;
use inkwell::values::AnyValue;
use inkwell::IntPredicate;

impl<'a> CodeGenerator<'a> {
    pub fn gen_unary_expr(&mut self, unary_expr: &UnaryExprAst) -> Value<'a> {
        let ty_id = self.resolve[unary_expr.expr_id].ty_id;

        match unary_expr.op {
            UnaryOpKind::Minus => {
                let value = self.gen_rvalue_expr(&unary_expr.expr).unwrap();

                if ty_id.is_int() {
                    self.builder
                        .build_int_neg(value.into_int_value(), "")
                        .as_any_value_enum()
                        .into()
                } else {
                    debug_assert!(ty_id.is_float());

                    self.builder
                        .build_float_neg(value.into_float_value(), "")
                        .as_any_value_enum()
                        .into()
                }
            }
            UnaryOpKind::Not => {
                let value = self
                    .gen_rvalue_expr(&unary_expr.expr)
                    .unwrap()
                    .into_int_value();

                if ty_id.is_int() {
                    self.builder.build_not(value, "").as_any_value_enum().into()
                } else {
                    debug_assert!(ty_id == tys::BOOL);

                    let value =
                        self.builder
                            .build_int_compare(IntPredicate::EQ, value, self.llvm_true, "");

                    self.builder
                        .build_select(value, self.llvm_false, self.llvm_true, "")
                        .as_any_value_enum()
                        .into()
                }
            }
            UnaryOpKind::Addr { .. } => {
                let value = self.gen_expr(&unary_expr.expr);

                match value {
                    Value::Lvalue { pointer, .. } => pointer.as_any_value_enum().into(),
                    _ => todo!(),
                }
            }
        }
    }
}
