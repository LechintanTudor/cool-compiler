use crate::{BuilderExt, CodeGenerator, LoadedValue, Value};
use cool_ast::IndexExprAst;
use cool_lexer::sym;
use cool_resolve::{SliceTy, ValueTy};
use inkwell::values::IntValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_index_expr(&mut self, expr: &IndexExprAst) -> Value<'a> {
        let base = self.gen_expr(&expr.base, None);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let index = self.gen_loaded_expr(&expr.index);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let index = index.as_basic_value().unwrap().into_int_value();

        match self.resolve[expr.base.expr_id()].ty_id.shape.get_value() {
            ValueTy::Array(_) => self.continue_gen_array_index_expr(expr, base, index),
            ValueTy::ManyPtr(_) => self.continue_gen_many_ptr_index_expr(expr, base, index),
            ValueTy::Slice(_) => self.continue_gen_slice_index_expr(expr, base, index),
            _ => unreachable!(),
        }
    }

    pub fn continue_gen_array_index_expr(
        &mut self,
        _expr: &IndexExprAst,
        base: Value<'a>,
        index: IntValue<'a>,
    ) -> Value<'a> {
        match base {
            Value::Void => Value::Void,
            Value::Memory(memory) => {
                let elem_ty = memory.ty.into_array_type().get_element_type();
                let elem_ptr = unsafe { self.builder.build_gep(elem_ty, memory.ptr, &[index], "") };
                Value::memory(elem_ptr, elem_ty)
            }
            Value::Register(array_value) => {
                let array_ptr = self.util_gen_init(array_value);
                let elem_ty = array_value.get_type().into_array_type().get_element_type();
                let elem_ptr = unsafe { self.builder.build_gep(elem_ty, array_ptr, &[index], "") };
                Value::memory(elem_ptr, elem_ty)
            }
            Value::Fn(_) => unreachable!(),
        }
    }

    pub fn continue_gen_many_ptr_index_expr(
        &mut self,
        expr: &IndexExprAst,
        base: Value<'a>,
        index: IntValue<'a>,
    ) -> Value<'a> {
        let elem_ty_id = self
            .resolve
            .get_expr_ty_id(expr.base.expr_id())
            .shape
            .get_many_ptr()
            .pointee;

        let elem_ty = self.tys[elem_ty_id].unwrap();

        match self.gen_loaded_value(base) {
            LoadedValue::Void => Value::Void,
            LoadedValue::Register(ptr_value) => {
                let ptr_value = ptr_value.into_pointer_value();
                let ptr_value = unsafe { self.builder.build_gep(elem_ty, ptr_value, &[index], "") };
                Value::memory(ptr_value, elem_ty)
            }
        }
    }

    pub fn continue_gen_slice_index_expr(
        &mut self,
        expr: &IndexExprAst,
        base: Value<'a>,
        index: IntValue<'a>,
    ) -> Value<'a> {
        let slice_ty_id = self.resolve.get_expr_ty_id(expr.base.expr_id());
        let elem_ty_id = slice_ty_id.shape.get_slice().elem;
        let elem_ty = self.tys[elem_ty_id].unwrap();

        match base {
            Value::Void => Value::Void,
            Value::Memory(memory) => {
                let ptr_value = self
                    .util_gen_loaded_field(slice_ty_id, memory.ptr, sym::PTR)
                    .into_basic_value()
                    .into_pointer_value();

                let ptr_value = unsafe { self.builder.build_gep(elem_ty, ptr_value, &[index], "") };
                Value::memory(ptr_value, elem_ty)
            }
            Value::Register(slice_value) => {
                let slice_value = slice_value.into_struct_value();

                let ptr_value = self
                    .builder
                    .build_extract_value(slice_value, SliceTy::PTR_FIELD_INDEX, "")
                    .unwrap()
                    .into_pointer_value();

                let ptr_value = unsafe { self.builder.build_gep(elem_ty, ptr_value, &[index], "") };
                Value::memory(ptr_value, elem_ty)
            }
            Value::Fn(_) => unreachable!(),
        }
    }
}
