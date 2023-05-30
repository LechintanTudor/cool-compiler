use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::SubscriptExprAst;
use cool_resolve::{tys, TyId, ValueTy};
use inkwell::types::BasicType;
use inkwell::values::{BasicValue, BasicValueEnum};

const SLICE_PTR_FIELD_INDEX: u32 = 0;
const SLICE_LEN_FIELD_INDEX: u32 = 1;

impl<'a> CodeGenerator<'a> {
    pub fn gen_subscript_expr(&mut self, expr: &SubscriptExprAst) -> Value<'a> {
        let base_ty = &self.resolve[self.resolve[expr.base.expr_id()].ty_id].ty;
        let subscript_ty_id = self.resolve[expr.subscript.expr_id()].ty_id;

        match base_ty {
            ValueTy::Array(_) => {
                if subscript_ty_id == tys::USIZE {
                    self.gen_array_elem_subscript_expr(expr)
                } else {
                    self.gen_array_slice_subscript_expr(expr)
                }
            }
            ValueTy::ManyPtr(_) => {
                todo!()
            }
            ValueTy::Slice(_) => {
                if subscript_ty_id == tys::USIZE {
                    self.gen_slice_elem_subscript_expr(expr)
                } else {
                    todo!()
                }
            }
            _ => unreachable!("unsuported subscript operation"),
        }
    }

    fn gen_array_elem_subscript_expr(&mut self, expr: &SubscriptExprAst) -> Value<'a> {
        let base = self.gen_expr(&expr.base, None);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let subscript = self.gen_loaded_expr(&expr.subscript);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let subscript = subscript.as_basic_value().unwrap().into_int_value();

        match base {
            Value::Void => Value::Void,
            Value::Memory(memory) => {
                let elem_ty = memory.ty.into_array_type().get_element_type();
                let elem_ptr = unsafe {
                    self.builder
                        .build_gep(elem_ty, memory.ptr, &[subscript], "")
                };

                Value::memory(elem_ptr, elem_ty)
            }
            Value::Register(array_value) => {
                let array_ptr = self.util_gen_init(array_value);

                let elem_ty = array_value.get_type().into_array_type().get_element_type();
                let elem_ptr =
                    unsafe { self.builder.build_gep(elem_ty, array_ptr, &[subscript], "") };

                Value::memory(elem_ptr, elem_ty)
            }
            Value::Fn(_) => unreachable!(),
        }
    }

    fn gen_array_slice_subscript_expr(&mut self, expr: &SubscriptExprAst) -> Value<'a> {
        let base = self.gen_expr(&expr.base, None);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        self.gen_loaded_expr(&expr.subscript);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let array_ty_id = self.resolve[expr.base.expr_id()].ty_id;
        let array_ty = self.resolve[array_ty_id].ty.as_array().unwrap();

        let expr_ty_id = self.resolve[expr.expr_id].ty_id;
        let elem_ty_id = self.resolve[expr_ty_id].ty.as_slice().unwrap().elem;

        match base {
            Value::Void => todo!(),
            Value::Memory(memory) => {
                let elem_ty = self.tys[elem_ty_id].unwrap();
                let index = self.tys.isize_ty().const_zero();

                let ptr = unsafe { self.builder.build_gep(elem_ty, memory.ptr, &[index], "") }
                    .as_basic_value_enum();

                let len = self
                    .tys
                    .isize_ty()
                    .const_int(array_ty.len, false)
                    .as_basic_value_enum();

                self.util_gen_slice(expr_ty_id, ptr, len)
            }
            Value::Register(_) => todo!(),
            Value::Fn(_) => unreachable!(),
        }
    }

    fn gen_slice_elem_subscript_expr(&mut self, expr: &SubscriptExprAst) -> Value<'a> {
        let base = self.gen_expr(&expr.base, None);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        self.gen_loaded_expr(&expr.subscript);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let slice_ty_id = self.resolve[expr.base.expr_id()].ty_id;
        let _slice_ty = self.resolve[slice_ty_id].ty.as_slice().unwrap();

        let expr_ty_id = self.resolve[expr.expr_id].ty_id;
        let _elem_ty_id = self.resolve[expr_ty_id].ty.as_slice().unwrap().elem;

        match base {
            Value::Void => todo!(),
            Value::Memory(_memory) => {
                todo!()
            }
            Value::Register(_) => todo!(),
            Value::Fn(_) => unreachable!(),
        }
    }

    fn util_gen_dangling_ptr(&mut self, pointee_ty_id: TyId) -> BasicValueEnum<'a> {
        let pointee_ty = &self.resolve[pointee_ty_id];
        let ptr_int_value = self.tys.isize_ty().const_int(pointee_ty.align, false);

        match self.tys[pointee_ty_id] {
            Some(pointee_ty) => {
                let ptr_ty = pointee_ty.ptr_type(Default::default());

                self.builder
                    .build_int_to_ptr(ptr_int_value, ptr_ty, "")
                    .as_basic_value_enum()
            }
            None => ptr_int_value.as_basic_value_enum(),
        }
    }

    fn util_gen_slice(
        &mut self,
        slice_ty_id: TyId,
        ptr: BasicValueEnum<'a>,
        len: BasicValueEnum<'a>,
    ) -> Value<'a> {
        let slice_ty = self.tys[slice_ty_id].unwrap();
        let slice_ptr = self.util_gen_alloca(slice_ty);

        let ptr_ptr = self
            .builder
            .build_struct_gep(slice_ty, slice_ptr, SLICE_PTR_FIELD_INDEX, "")
            .unwrap();
        self.builder.build_store(ptr_ptr, ptr);

        let len_ptr = self
            .builder
            .build_struct_gep(slice_ty, slice_ptr, SLICE_LEN_FIELD_INDEX, "")
            .unwrap();
        self.builder.build_store(len_ptr, len);

        Value::memory(slice_ptr, slice_ty)
    }
}
