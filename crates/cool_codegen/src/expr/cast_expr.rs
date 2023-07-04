use crate::{BuilderExt, CodeGenerator, LoadedValue};
use cool_ast::{CastExprAst, CastKind};
use inkwell::values::{BasicValue, BasicValueEnum};

impl<'a> CodeGenerator<'a> {
    pub fn gen_cast_expr(&mut self, expr: &CastExprAst) -> LoadedValue<'a> {
        let from_value = self.gen_loaded_expr(&expr.base);
        if self.builder.current_block_diverges() {
            return LoadedValue::None;
        }

        let from_value = from_value.unwrap();
        let from_ty_id = expr.from_ty_id();
        let to_ty_id = expr.to_ty_id();
        let to_ty = self.tys[to_ty_id].unwrap();

        let value: BasicValueEnum<'a> = match expr.kind {
            CastKind::IntToFloat => {
                if from_ty_id.is_signed_int() {
                    self.builder
                        .build_signed_int_to_float(
                            from_value.into_int_value(),
                            to_ty.into_float_type(),
                            "",
                        )
                        .as_basic_value_enum()
                } else {
                    self.builder
                        .build_unsigned_int_to_float(
                            from_value.into_int_value(),
                            to_ty.into_float_type(),
                            "",
                        )
                        .as_basic_value_enum()
                }
            }
            CastKind::IntToInt => {
                self.builder
                    .build_int_cast_sign_flag(
                        from_value.into_int_value(),
                        to_ty.into_int_type(),
                        from_ty_id.is_signed_int(),
                        "",
                    )
                    .as_basic_value_enum()
            }
            CastKind::IntToPtr => {
                self.builder
                    .build_int_to_ptr(from_value.into_int_value(), to_ty.into_pointer_type(), "")
                    .as_basic_value_enum()
            }
            CastKind::FloatToFloat => {
                self.builder
                    .build_float_cast(from_value.into_float_value(), to_ty.into_float_type(), "")
                    .as_basic_value_enum()
            }
            CastKind::FloatToInt => {
                if to_ty_id.is_signed_int() {
                    self.builder
                        .build_float_to_signed_int(
                            from_value.into_float_value(),
                            to_ty.into_int_type(),
                            "",
                        )
                        .as_basic_value_enum()
                } else {
                    self.builder
                        .build_float_to_unsigned_int(
                            from_value.into_float_value(),
                            to_ty.into_int_type(),
                            "",
                        )
                        .as_basic_value_enum()
                }
            }
            CastKind::PtrToPtr | CastKind::TupleToSlice => {
                self.builder.build_bitcast(from_value, to_ty, "")
            }
            CastKind::PtrToUsize => {
                self.builder
                    .build_ptr_to_int(from_value.into_pointer_value(), to_ty.into_int_type(), "")
                    .as_basic_value_enum()
            }
        };

        Some(value)
    }
}
