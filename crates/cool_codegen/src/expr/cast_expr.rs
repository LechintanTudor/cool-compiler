use crate::{BuilderExt, CodeGenerator, LoadedValue};
use cool_ast::{CastExprAst, CastKind};
use inkwell::values::BasicValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_cast_expr(&mut self, expr: &CastExprAst) -> LoadedValue<'a> {
        let base_value = self.gen_loaded_expr(&expr.base);
        if self.builder.current_block_diverges() {
            return LoadedValue::None;
        }

        match expr.kind {
            CastKind::PtrToPtr | CastKind::TupleToSlice => {
                let expr_ty = self.tys[expr.expr_id.ty_id].unwrap();

                self.builder
                    .build_bitcast(base_value.unwrap(), expr_ty, "")
                    .into()
            }
            CastKind::PtrToUsize => {
                let base_value = base_value.unwrap().into_pointer_value();

                self.builder
                    .build_ptr_to_int(base_value, self.tys.isize_ty(), "")
                    .as_basic_value_enum()
                    .into()
            }
        }
    }
}
