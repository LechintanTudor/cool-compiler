use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::IndexExprAst;
use cool_lexer::sym;
use cool_resolve::ValueTy;
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

        let index = index.unwrap().into_int_value();

        match expr.base.expr_id().ty_id.get_value() {
            ValueTy::Array(_) => self.continue_gen_array_index_expr(expr, base, index),
            ValueTy::ManyPtr(_) => self.continue_gen_many_ptr_index_expr(expr, base, index),
            ValueTy::Slice(_) => self.continue_gen_slice_index_expr(expr, base, index),
            _ => panic!("value is not indexable"),
        }
    }

    pub fn continue_gen_array_index_expr(
        &mut self,
        expr: &IndexExprAst,
        base: Value<'a>,
        index: IntValue<'a>,
    ) -> Value<'a> {
        let elem_ty_id = expr.base.expr_id().ty_id.get_array().elem;
        let elem_ty = self.tys[elem_ty_id].unwrap();

        match base {
            Value::Void => Value::Void,
            Value::Memory(memory) => {
                let elem_ptr = unsafe { self.builder.build_gep(elem_ty, memory, &[index], "") };
                Value::Memory(elem_ptr)
            }
            Value::Register(array_value) => {
                let array_ptr = self.util_gen_init(array_value);
                let elem_ptr = unsafe { self.builder.build_gep(elem_ty, array_ptr, &[index], "") };
                Value::Memory(elem_ptr)
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
        let many_ptr_ty_id = expr.base.expr_id().ty_id;

        let many_ptr = match self.gen_loaded_value(many_ptr_ty_id, base) {
            Some(many_ptr) => many_ptr.into_pointer_value(),
            None => return Value::Void,
        };

        let pointee_ty_id = expr.expr_id.ty_id;

        let ptr_value = match self.tys[pointee_ty_id] {
            Some(pointee_ty) => unsafe {
                self.builder.build_gep(pointee_ty, many_ptr, &[index], "")
            },
            None => {
                let pointee_align = self.resolve.get_ty_def(pointee_ty_id).unwrap().align;
                let pointee_align_value = self.tys.isize_ty().const_int(pointee_align, false);

                self.builder
                    .build_int_to_ptr(pointee_align_value, self.tys.i8_ptr_ty(), "")
            }
        };

        Value::Memory(ptr_value)
    }

    pub fn continue_gen_slice_index_expr(
        &mut self,
        expr: &IndexExprAst,
        base: Value<'a>,
        index: IntValue<'a>,
    ) -> Value<'a> {
        let slice_ty_id = expr.base.expr_id().ty_id;
        let elem_ty_id = slice_ty_id.get_slice().elem;
        let elem_ty = self.tys[elem_ty_id].unwrap();

        match base {
            Value::Void => Value::Void,
            Value::Memory(memory) => {
                let ptr_value = self
                    .util_gen_loaded_field(slice_ty_id, memory, sym::PTR)
                    .unwrap()
                    .into_pointer_value();

                let ptr_value = unsafe { self.builder.build_gep(elem_ty, ptr_value, &[index], "") };
                Value::Memory(ptr_value)
            }
            Value::Register(slice_value) => {
                let slice_value = slice_value.into_struct_value();

                let ptr_value = self
                    .builder
                    .build_extract_value(slice_value, 0, "")
                    .unwrap()
                    .into_pointer_value();

                let ptr_value = unsafe { self.builder.build_gep(elem_ty, ptr_value, &[index], "") };
                Value::Memory(ptr_value)
            }
            Value::Fn(_) => unreachable!(),
        }
    }
}
