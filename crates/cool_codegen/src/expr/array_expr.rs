use crate::{AnyValueEnumExt, CodeGenerator, Value};
use cool_ast::ArrayExprAst;
use inkwell::values::AnyValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_array_expr(&mut self, expr: &ArrayExprAst) -> Value<'a> {
        let ty_id = self.resolve[expr.expr_id].ty_id;

        if self.resolve.is_ty_id_zst(ty_id) {
            return Value::Void;
        }

        let mut array_value = self.tys[ty_id].into_array_type().get_undef();

        for (i, elem) in expr.elems.iter().enumerate() {
            let elem_value = self.gen_rvalue_expr(elem).unwrap().into_basic_value();

            array_value = self
                .builder
                .build_insert_value(array_value, elem_value, i as u32, "")
                .unwrap()
                .into_array_value();
        }

        array_value.as_any_value_enum().into()
    }
}
