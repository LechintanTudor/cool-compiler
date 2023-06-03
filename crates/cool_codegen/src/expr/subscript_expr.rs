use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::SubscriptExprAst;
use cool_resolve::{tys, ValueTy};
use inkwell::values::BasicValue;

const SLICE_PTR_FIELD_OFFSET: u32 = 0;
const SLICE_LEN_FIELD_OFFSET: u32 = 1;

impl<'a> CodeGenerator<'a> {
    pub fn gen_subscript_expr(&mut self, expr: &SubscriptExprAst) -> Value<'a> {
        let base_ty = &self.resolve[self.resolve[expr.base.expr_id()].ty_id].ty;
        let subscript_ty_id = self.resolve[expr.subscript.expr_id()].ty_id;

        match base_ty {
            ValueTy::Array(_) => {
                if subscript_ty_id == tys::USIZE {
                    self.gen_array_elem_subscript_expr(expr)
                } else {
                    self.gen_array_full_slice_subscript_expr(expr)
                }
            }
            ValueTy::ManyPtr(_) => {
                todo!()
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

    fn gen_array_full_slice_subscript_expr(&mut self, expr: &SubscriptExprAst) -> Value<'a> {
        let base = self.gen_expr(&expr.base, None);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        self.gen_loaded_expr(&expr.subscript);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let ptr_value = match base {
            Value::Void => todo!(),
            Value::Memory(memory) => memory.ptr,
            Value::Register(_) => todo!(),
            Value::Fn(_) => unreachable!(),
        };

        let len = self
            .resolve
            .get_expr_ty(expr.base.expr_id())
            .ty
            .as_array()
            .unwrap()
            .len;

        let len_value = self
            .tys
            .isize_ty()
            .const_int(len, false)
            .as_basic_value_enum();

        let slice_ty_id = self.resolve.get_expr_ty_id(expr.expr_id);
        let slice_ty = self.tys[slice_ty_id].unwrap();
        let slice_ptr = self.util_gen_alloca(slice_ty);

        let ptr_ptr = self
            .builder
            .build_struct_gep(slice_ty, slice_ptr, SLICE_PTR_FIELD_OFFSET, "")
            .unwrap();
        self.builder.build_store(ptr_ptr, ptr_value);

        let len_ptr = self
            .builder
            .build_struct_gep(slice_ty, slice_ptr, SLICE_LEN_FIELD_OFFSET, "")
            .unwrap();
        self.builder.build_store(len_ptr, len_value);

        Value::memory(slice_ptr, slice_ty)
    }
}
