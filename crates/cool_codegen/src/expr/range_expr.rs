use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::RangeExprAst;
use cool_lexer::sym;
use cool_resolve::{TyId, ValueTy};
use inkwell::values::{BasicValueEnum, IntValue, PointerValue};

impl<'a> CodeGenerator<'a> {
    pub fn gen_range_expr(
        &mut self,
        expr: &RangeExprAst,
        memory: Option<PointerValue<'a>>,
    ) -> Value<'a> {
        // Base
        let base = self.gen_expr(&expr.base, None);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let (from, to) = expr.kind.as_from_to_pair();

        // From
        let from = from
            .and_then(|from| self.gen_loaded_expr(from))
            .map(BasicValueEnum::into_int_value)
            .unwrap_or_else(|| self.tys.isize_ty().const_zero());

        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        // To
        let to = to
            .and_then(|to| self.gen_loaded_expr(to))
            .map(BasicValueEnum::into_int_value);

        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        // Slice
        let memory = memory.unwrap_or_else(|| {
            let slice_ty_id = expr.expr_id.ty_id;
            let slice_ty = self.tys[slice_ty_id].unwrap();
            self.util_gen_alloca(slice_ty)
        });

        match expr.base.expr_id().ty_id.as_value().unwrap() {
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
        to: Option<IntValue<'a>>,
        memory: PointerValue<'a>,
    ) -> Value<'a> {
        let ptr_value = match base {
            Value::Void => todo!("handle zst array"),
            Value::Fn(_) => unreachable!(),
            Value::Register(value) => unsafe {
                let memory = self.util_gen_init(value);
                let elem_ty_id = expr.expr_id.ty_id.get_array().elem;

                match self.tys[elem_ty_id] {
                    Some(elem_ty) => unsafe {
                        self.builder.build_gep(elem_ty, memory, &[from], "")
                    },
                    None => memory,
                }
            },
            Value::Memory(memory) => {
                let elem_ty_id = expr.expr_id.ty_id.get_array().elem;

                match self.tys[elem_ty_id] {
                    Some(elem_ty) => unsafe {
                        self.builder.build_gep(elem_ty, memory, &[from], "")
                    },
                    None => memory,
                }
            }
        };

        let to = to.unwrap_or_else(|| {
            let base_len = expr.base.expr_id().ty_id.get_array().len;
            self.tys.isize_ty().const_int(base_len, false)
        });

        let len_value = self.builder.build_int_sub(to, from, "");
        self.util_gen_slice(expr.expr_id.ty_id, memory, ptr_value, len_value);
        Value::Memory(memory)
    }

    fn gen_slice_range_expr(
        &mut self,
        expr: &RangeExprAst,
        base: Value<'a>,
        from: IntValue<'a>,
        to: Option<IntValue<'a>>,
        memory: PointerValue<'a>,
    ) -> Value<'a> {
        let slice_ty_id = expr.base.expr_id().ty_id;
        let elem_ty_id = slice_ty_id.get_slice().elem;
        let elem_ty = self.tys[elem_ty_id].unwrap();

        let (ptr_value, len_value) = match base {
            Value::Void => todo!("handle zst array"),
            Value::Fn(_) => unreachable!(),
            Value::Register(slice_value) => {
                let slice_value = slice_value.into_struct_value();

                let ptr_value = self
                    .builder
                    .build_extract_value(slice_value, 0, "")
                    .unwrap()
                    .into_pointer_value();

                let ptr_value = unsafe { self.builder.build_gep(elem_ty, ptr_value, &[from], "") };

                let to = to.unwrap_or_else(|| {
                    self.builder
                        .build_extract_value(slice_value, 1, "")
                        .unwrap()
                        .into_int_value()
                });

                let len_value = self.builder.build_int_sub(to, from, "");

                (ptr_value, len_value)
            }
            Value::Memory(slice_memory) => {
                let ptr_value = self
                    .util_gen_loaded_field(slice_ty_id, slice_memory, sym::PTR)
                    .unwrap()
                    .into_pointer_value();

                let ptr_value = unsafe { self.builder.build_gep(elem_ty, ptr_value, &[from], "") };

                let to = to.unwrap_or_else(|| {
                    self.util_gen_loaded_field(slice_ty_id, slice_memory, sym::LEN)
                        .unwrap()
                        .into_int_value()
                });

                let len_value = self.builder.build_int_sub(to, from, "");

                (ptr_value, len_value)
            }
        };

        self.util_gen_slice(slice_ty_id, memory, ptr_value, len_value);
        Value::Memory(memory)
    }

    fn util_gen_slice(
        &self,
        slice_ty_id: TyId,
        memory: PointerValue<'a>,
        ptr_value: PointerValue<'a>,
        len_value: IntValue<'a>,
    ) {
        let slice_ty = match self.tys[slice_ty_id] {
            Some(slice_ty) => slice_ty,
            None => return,
        };

        let field_map = self.tys.get_field_map(slice_ty_id);

        if let Some(ptr_field_index) = field_map.get(sym::PTR) {
            let ptr_field_ptr = self
                .builder
                .build_struct_gep(slice_ty, memory, ptr_field_index, "")
                .unwrap();

            self.builder.build_store(ptr_field_ptr, ptr_value);
        }

        if let Some(len_field_index) = field_map.get(sym::LEN) {
            let len_field_ptr = self
                .builder
                .build_struct_gep(slice_ty, memory, len_field_index, "")
                .unwrap();

            self.builder.build_store(len_field_ptr, len_value);
        }
    }
}
