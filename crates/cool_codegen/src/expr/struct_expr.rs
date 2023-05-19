use crate::{CodeGenerator, Value};
use cool_ast::StructExprAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_struct_expr(&mut self, expr: &StructExprAst) -> Value<'a> {
        let expr_ty_id = self.resolve[expr.expr_id].ty_id;
        let struct_ty = self.tys[expr_ty_id].unwrap();
        let struct_ptr = self.util_gen_alloca(struct_ty);

        for initializer in expr.initializers.iter() {
            let field_value = self.gen_loaded_expr(&initializer.expr);

            let Some(index) = self
                .tys
                .get_field_map(expr_ty_id)
                .get(initializer.ident.symbol) else {
                    continue;
                };

            let field_ptr = self
                .builder
                .build_struct_gep(struct_ty, struct_ptr, index, "")
                .unwrap();

            self.builder
                .build_store(field_ptr, field_value.into_basic_value());
        }

        Value::memory(struct_ptr, struct_ty)
    }
}
