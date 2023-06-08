use crate::{BuilderExt, CodeGenerator, LoadedValue, MemoryValue, Value};
use cool_ast::RangeExprAst;
use cool_lexer::sym;
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_range_expr(
        &mut self,
        expr: &RangeExprAst,
        memory: Option<MemoryValue<'a>>,
    ) -> Value<'a> {
        self.gen_array_range_expr(expr, memory)
    }

    fn gen_array_range_expr(
        &mut self,
        expr: &RangeExprAst,
        memory: Option<MemoryValue<'a>>,
    ) -> Value<'a> {
        let base = self.gen_expr(&expr.base, None);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let (from, to) = expr.kind.as_from_to_pair();

        let from = from
            .map(|from| self.gen_loaded_expr(from))
            .unwrap_or(LoadedValue::Void);

        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let to = to
            .map(|to| self.gen_loaded_expr(to))
            .unwrap_or(LoadedValue::Void);

        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let slice_ty_id = self.resolve.get_expr_ty_id(expr.expr_id);

        let memory = memory.unwrap_or_else(|| {
            let slice_ty = self.tys[slice_ty_id].unwrap();
            let slice_ptr = self.util_gen_alloca(slice_ty);
            MemoryValue::new(slice_ptr, slice_ty)
        });

        let base_len = self
            .resolve
            .get_expr_ty(expr.base.expr_id())
            .ty
            .as_array()
            .unwrap()
            .len;

        let elem_ty = self.tys[self.resolve.get_expr_ty_id(expr.base.expr_id())]
            .unwrap()
            .into_array_type()
            .get_element_type();

        let from_value = match from {
            LoadedValue::Register(value) => value.into_int_value(),
            LoadedValue::Void => self.tys.isize_ty().const_zero(),
        };

        let ptr_value = match base {
            Value::Void => todo!("handle zst array"),
            Value::Fn(_) => unreachable!(),
            Value::Register(value) => unsafe {
                let array_ptr = self.util_gen_init(value);
                self.builder
                    .build_gep(elem_ty, array_ptr, &[from_value], "")
            },
            Value::Memory(array_memory) => unsafe {
                self.builder
                    .build_gep(elem_ty, array_memory.ptr, &[from_value], "")
            },
        };

        let ptr_field_index = self.tys.get_field_map(slice_ty_id)[sym::PTR];
        let ptr_field_ptr = self
            .builder
            .build_struct_gep(memory.ty, memory.ptr, ptr_field_index, "")
            .unwrap();

        self.builder.build_store(ptr_field_ptr, ptr_value);

        let to_value = match to {
            LoadedValue::Register(value) => value.into_int_value(),
            LoadedValue::Void => self.tys.isize_ty().const_int(base_len, false),
        };

        let len_value = self
            .builder
            .build_int_sub(to_value, from_value, "")
            .as_basic_value_enum();

        let len_field_index = self.tys.get_field_map(slice_ty_id)[sym::LEN];
        let len_field_ptr = self
            .builder
            .build_struct_gep(memory.ty, memory.ptr, len_field_index, "")
            .unwrap();

        self.builder.build_store(len_field_ptr, len_value);

        memory.into()
    }
}
