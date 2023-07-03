use crate::{BuilderExt, CodeGenerator, LoadedValue, Value};
use cool_ast::VariantWrapExprAst;
use cool_lexer::sym;
use cool_resolve::TaggedUnionKind;
use inkwell::values::PointerValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_variant_wrap_expr(
        &mut self,
        expr: &VariantWrapExprAst,
        memory: Option<PointerValue<'a>>,
    ) -> Value<'a> {
        let expr_ty_id = expr.expr_id.ty_id;
        let expr_ty = self.tys[expr_ty_id].unwrap();

        let memory = memory.unwrap_or_else(|| {
            let struct_ty = self.tys[expr_ty_id].unwrap();
            self.util_gen_alloca(struct_ty)
        });

        let inner_expr_value = self.gen_loaded_expr(&expr.inner);
        if self.builder.current_block_diverges() {
            return Value::Void;
        }

        let tagged_union_kind = self
            .resolve
            .get_ty_def(expr_ty_id)
            .unwrap()
            .kind
            .as_tagged_union()
            .unwrap()
            .kind;

        match tagged_union_kind {
            TaggedUnionKind::Basic { .. } => {
                if let LoadedValue::Some(value) = inner_expr_value {
                    self.builder.build_store(memory, value);
                }

                let index_field_index = self
                    .tys
                    .get_field_map(expr_ty_id)
                    .get(sym::VARIANT_INDEX)
                    .unwrap();

                let index_field_ptr = self
                    .builder
                    .build_struct_gep(expr_ty, memory, index_field_index, "")
                    .unwrap();

                let index_field_value = self
                    .tys
                    .i8_ty()
                    .const_int(expr.variant_index() as u64, false);

                self.builder.build_store(index_field_ptr, index_field_value);
            }
            TaggedUnionKind::NullablePtr => {
                match inner_expr_value {
                    LoadedValue::Some(value) => {
                        println!("{:#?}", value);
                        self.builder.build_store(memory, value);
                    }
                    LoadedValue::None => {
                        self.builder
                            .build_store(memory, self.tys.isize_ty().const_zero());
                    }
                }
            }
        }

        Value::Memory(memory)
    }
}
