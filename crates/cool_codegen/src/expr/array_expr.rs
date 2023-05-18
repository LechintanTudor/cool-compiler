use crate::{BaiscTypeEnumOptionExt, CodeGenerator, Value};
use cool_ast::{ArrayExprAst, ArrayRepeatExprAst};
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_array_expr(&mut self, expr: &ArrayExprAst) -> Value<'a> {
        let ty_id = self.resolve[expr.expr_id].ty_id;

        if self.resolve.is_ty_id_zst(ty_id) {
            return Value::Void;
        }

        let array_type = self.tys[ty_id].into_array_type();
        let array_pointer = self.util_gen_alloca(array_type.get_undef(), "");

        let index_type = self.tys.isize_ty();
        let elem_type = array_type.get_element_type();

        for (i, elem) in expr.elems.iter().enumerate() {
            let elem_index = index_type.const_int(i as u64, false);
            let elem_value = self.gen_loaded_expr(elem).into_basic_value();

            let elem_pointer = unsafe {
                self.builder
                    .build_gep(elem_type, array_pointer, &[elem_index], "")
            };

            self.builder.build_store(elem_pointer, elem_value);
        }

        self.builder
            .build_load(array_type, array_pointer, "")
            .as_basic_value_enum()
            .into()
    }

    pub fn gen_array_repeat_expr(&mut self, expr: &ArrayRepeatExprAst) -> Value<'a> {
        let elem_value = self.gen_loaded_expr(&expr.elem).into_basic_value();

        let ty_id = self.resolve[expr.expr_id].ty_id;

        if self.resolve.is_ty_id_zst(ty_id) {
            return Value::Void;
        }

        let array_type = self.tys[ty_id].into_array_type();
        let array_pointer = self.util_gen_alloca(array_type.get_undef(), "");

        let index_type = self.tys.isize_ty();
        let elem_type = elem_value.get_type();

        for i in 0..expr.len {
            let elem_index = index_type.const_int(i, false);

            let elem_pointer = unsafe {
                self.builder
                    .build_gep(elem_type, array_pointer, &[elem_index], "")
            };

            self.builder.build_store(elem_pointer, elem_value);
        }

        self.builder
            .build_load(array_type, array_pointer, "")
            .as_basic_value_enum()
            .into()
    }
}
