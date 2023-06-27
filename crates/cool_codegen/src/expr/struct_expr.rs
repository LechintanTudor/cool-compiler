use crate::{BuilderExt, CodeGenerator, MemoryValue, Value};
use cool_ast::StructExprAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_struct_expr(
        &mut self,
        expr: &StructExprAst,
        memory: Option<MemoryValue<'a>>,
    ) -> Value<'a> {
        let expr_ty_id = self.resolve[expr.expr_id].ty_id;

        let memory = memory.unwrap_or_else(|| {
            let struct_ty = self.tys[expr_ty_id].unwrap();
            let struct_ptr = self.util_gen_alloca(struct_ty);
            MemoryValue::new(struct_ptr, struct_ty)
        });

        let fields = self
            .resolve
            .get_ty_def(expr_ty_id)
            .unwrap()
            .get_aggregate_fields()
            .unwrap();

        for initializer in expr.initializers.iter() {
            let Some(field_index) = self
                .tys
                .get_field_map(expr_ty_id)
                .get(initializer.ident.symbol) else {
                    self.gen_expr(&initializer.expr, None);
                    continue;
                };

            let field_ty = fields
                .iter()
                .find(|field| field.symbol == initializer.ident.symbol)
                .and_then(|field| self.tys[field.ty_id])
                .unwrap();

            let field_ptr = self
                .builder
                .build_struct_gep(memory.ty, memory.ptr, field_index, "")
                .unwrap();

            let field_memory = Some(MemoryValue::new(field_ptr, field_ty));

            let field_expr = self.gen_expr(&initializer.expr, field_memory);
            if self.builder.current_block_diverges() {
                return Value::Void;
            }

            match field_expr {
                Value::Fn(fn_value) => {
                    self.builder
                        .build_store(field_ptr, fn_value.as_global_value().as_pointer_value());
                }
                Value::Register(value) => {
                    self.builder.build_store(field_ptr, value);
                }
                _ => (),
            }
        }

        Value::Memory(memory)
    }
}
