use crate::{BuilderExt, CodeGenerator, LoadedValue, MemoryValue, Value};
use cool_ast::{ArrayExprAst, ArrayRepeatExprAst};
use inkwell::IntPredicate;

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

            let elem_value = {
                let elem_value = self.gen_loaded_expr(elem);
                if self.builder.current_block_diverges() {
                    return Value::Void;
                }

                elem_value.into_basic_value()
            };

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

        let start_block = self.builder.get_insert_block().unwrap();
        let cond_block = self.context.insert_basic_block_after(start_block, "");
        let body_block = self.context.insert_basic_block_after(cond_block, "");
        let end_block = self.context.insert_basic_block_after(body_block, "");

        let index_ty = self.tys.isize_ty();
        let elem_ty = memory.ty.into_array_type().get_element_type();

        let elem_index_ptr = self.util_gen_init(index_ty.const_zero());
        let array_len_value = index_ty.const_int(expr.len, false);
        self.builder.build_unconditional_branch(cond_block);

        // Condition block
        self.builder.position_at_end(cond_block);

        let elem_index_value = self
            .builder
            .build_load(index_ty, elem_index_ptr, "")
            .into_int_value();

        let should_continue = self.builder.build_int_compare(
            IntPredicate::ULT,
            elem_index_value,
            array_len_value,
            "",
        );

        self.builder
            .build_conditional_branch(should_continue, body_block, end_block);

        // Body block
        self.builder.position_at_end(body_block);

        let elem_ptr = unsafe {
            self.builder
                .build_gep(elem_ty, memory.ptr, &[elem_index_value], "")
        };

        self.builder.build_store(elem_ptr, elem_value);

        let next_elem_index_value =
            self.builder
                .build_int_add(elem_index_value, index_ty.const_int(1, false), "");

        self.builder
            .build_store(elem_index_ptr, next_elem_index_value);

        self.builder.build_unconditional_branch(cond_block);

        // End block
        self.builder.position_at_end(end_block);
        Value::Memory(memory)
    }
}
