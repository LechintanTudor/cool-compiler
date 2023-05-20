use crate::{CodeGenerator, LoadedValue, MemoryValue, Value};
use cool_ast::{ArrayExprAst, ArrayRepeatExprAst};

impl<'a> CodeGenerator<'a> {
    // TODO: Support ZST
    pub fn gen_array_expr(
        &mut self,
        expr: &ArrayExprAst,
        memory: Option<MemoryValue<'a>>,
    ) -> Value<'a> {
        let memory = memory.unwrap_or_else(|| {
            let ty_id = self.resolve[expr.expr_id].ty_id;
            let ty = self.tys[ty_id].unwrap();
            let ptr = self.util_gen_alloca(ty);
            MemoryValue::new(ptr, ty)
        });

        let index_ty = self.tys.isize_ty();
        let elem_ty = memory.ty.into_array_type().get_element_type();

        for (i, elem) in expr.elems.iter().enumerate() {
            let elem_index = index_ty.const_int(i as u64, false);
            let elem_value = self.gen_loaded_expr(elem).into_basic_value();

            let elem_pointer = unsafe {
                self.builder
                    .build_gep(elem_ty, memory.ptr, &[elem_index], "")
            };

            self.builder.build_store(elem_pointer, elem_value);
        }

        Value::Memory(memory)
    }

    pub fn gen_array_repeat_expr(
        &mut self,
        expr: &ArrayRepeatExprAst,
        memory: Option<MemoryValue<'a>>,
    ) -> Value<'a> {
        if expr.len == 0 {
            return Value::Void;
        }

        let LoadedValue::Register(elem_value) = self.gen_loaded_expr(&expr.elem) else {
            return Value::Void;
        };

        let memory = memory.unwrap_or_else(|| {
            let ty_id = self.resolve[expr.expr_id].ty_id;
            let ty = self.tys[ty_id].unwrap();
            let ptr = self.util_gen_alloca(ty);
            MemoryValue::new(ptr, ty)
        });

        let index_ty = self.tys.isize_ty();
        let elem_ty = memory.ty.into_array_type().get_element_type();

        for i in 0..expr.len {
            let elem_index = index_ty.const_int(i, false);

            let elem_ptr = unsafe {
                self.builder
                    .build_gep(elem_ty, memory.ptr, &[elem_index], "")
            };

            self.builder.build_store(elem_ptr, elem_value);
        }

        Value::Memory(memory)
    }
}
