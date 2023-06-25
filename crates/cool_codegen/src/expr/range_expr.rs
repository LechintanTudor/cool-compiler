use crate::{BuilderExt, CodeGenerator, LoadedValue, MemoryValue, Value};
use cool_ast::RangeExprAst;
use cool_lexer::sym;
use cool_resolve::{SliceTy, ValueTy};
use inkwell::values::{IntValue, PointerValue};

impl<'a> CodeGenerator<'a> {
    pub fn gen_range_expr(
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

        let from = match from {
            LoadedValue::Void => self.tys.isize_ty().const_zero(),
            LoadedValue::Register(value) => value.into_int_value(),
        };

        let to = to
            .map(|to| self.gen_loaded_expr(to))
            .unwrap_or(LoadedValue::Void);

        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let memory = memory.unwrap_or_else(|| {
            let slice_ty_id = self.resolve.get_expr_ty_id(expr.expr_id);
            let slice_ty = self.tys[slice_ty_id].unwrap();
            let slice_ptr = self.util_gen_alloca(slice_ty);
            MemoryValue::new(slice_ptr, slice_ty)
        });

        match self.resolve[expr.base.expr_id()]
            .ty_id
            .shape
            .as_value()
            .unwrap()
        {
            ValueTy::Array(_) => self.gen_array_range_expr(expr, base, from, to, memory),
            ValueTy::Slice(_) => self.gen_slice_range_expr(expr, base, from, to, memory),
            _ => unreachable!(),
        }
    }

    fn gen_array_range_expr(
        &mut self,
        expr: &RangeExprAst,
        base: Value<'a>,
        from: IntValue<'a>,
        to: LoadedValue<'a>,
        memory: MemoryValue<'a>,
    ) -> Value<'a> {
        let elem_ty = self.tys[self.resolve.get_expr_ty_id(expr.base.expr_id())]
            .unwrap()
            .into_array_type()
            .get_element_type();

        let ptr_value = match base {
            Value::Void => todo!("handle zst array"),
            Value::Fn(_) => unreachable!(),
            Value::Register(value) => unsafe {
                let array_ptr = self.util_gen_init(value);
                self.builder.build_gep(elem_ty, array_ptr, &[from], "")
            },
            Value::Memory(array_memory) => unsafe {
                self.builder
                    .build_gep(elem_ty, array_memory.ptr, &[from], "")
            },
        };

        let to = match to {
            LoadedValue::Register(value) => value.into_int_value(),
            LoadedValue::Void => {
                let base_len = self.resolve[expr.base.expr_id()]
                    .ty_id
                    .shape
                    .get_array()
                    .len;

                self.tys.isize_ty().const_int(base_len, false)
            }
        };

        let len_value = self.builder.build_int_sub(to, from, "");
        self.util_gen_slice(memory, ptr_value, len_value);
        memory.into()
    }

    fn gen_slice_range_expr(
        &mut self,
        expr: &RangeExprAst,
        base: Value<'a>,
        from: IntValue<'a>,
        to: LoadedValue<'a>,
        memory: MemoryValue<'a>,
    ) -> Value<'a> {
        let slice_ty_id = self.resolve.get_expr_ty_id(expr.base.expr_id());
        let elem_ty_id = slice_ty_id.shape.get_slice().elem;
        let elem_ty = self.tys[elem_ty_id].unwrap();

        let (ptr_value, len_value) = match base {
            Value::Void => todo!("handle zst array"),
            Value::Fn(_) => unreachable!(),
            Value::Register(slice_value) => {
                let slice_value = slice_value.into_struct_value();

                let ptr_value = self
                    .builder
                    .build_extract_value(slice_value, SliceTy::PTR_FIELD_INDEX, "")
                    .unwrap()
                    .into_pointer_value();

                let ptr_value = unsafe { self.builder.build_gep(elem_ty, ptr_value, &[from], "") };

                let len_value = self
                    .builder
                    .build_extract_value(slice_value, SliceTy::LEN_FIELD_INDEX, "")
                    .unwrap()
                    .into_int_value();

                let to = match to {
                    LoadedValue::Void => len_value,
                    LoadedValue::Register(value) => value.into_int_value(),
                };

                let len_value = self.builder.build_int_sub(to, from, "");

                (ptr_value, len_value)
            }
            Value::Memory(slice_memory) => {
                let ptr_value = self
                    .util_gen_loaded_field(slice_ty_id, slice_memory.ptr, sym::PTR)
                    .into_basic_value()
                    .into_pointer_value();

                let ptr_value = unsafe { self.builder.build_gep(elem_ty, ptr_value, &[from], "") };

                let len_value = self
                    .util_gen_loaded_field(slice_ty_id, slice_memory.ptr, sym::LEN)
                    .into_basic_value()
                    .into_int_value();

                let to = match to {
                    LoadedValue::Void => len_value,
                    LoadedValue::Register(value) => value.into_int_value(),
                };

                let len_value = self.builder.build_int_sub(to, from, "");

                (ptr_value, len_value)
            }
        };

        self.util_gen_slice(memory, ptr_value, len_value);
        memory.into()
    }

    fn util_gen_slice(
        &self,
        memory: MemoryValue<'a>,
        ptr_value: PointerValue<'a>,
        len_value: IntValue<'a>,
    ) {
        let ptr_ptr = self
            .builder
            .build_struct_gep(memory.ty, memory.ptr, SliceTy::PTR_FIELD_INDEX, "")
            .unwrap();

        self.builder.build_store(ptr_ptr, ptr_value);

        let len_ptr = self
            .builder
            .build_struct_gep(memory.ty, memory.ptr, SliceTy::LEN_FIELD_INDEX, "")
            .unwrap();

        self.builder.build_store(len_ptr, len_value);
    }
}
