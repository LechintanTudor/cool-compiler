use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::StructExprAst;
use inkwell::values::PointerValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_struct_expr(
        &mut self,
        expr: &StructExprAst,
        memory: Option<PointerValue<'a>>,
    ) -> Value<'a> {
        let struct_ty_id = expr.expr_id.ty_id;
        let struct_ty = self.tys[struct_ty_id];
        let memory = memory.or_else(|| struct_ty.map(|ty| self.util_gen_alloca(ty)));

        for initializer in expr.initializers.iter() {
            match (struct_ty, memory) {
                (Some(struct_ty), Some(memory)) => {
                    let Some(field_index) = self
                        .tys
                        .get_field_map(struct_ty_id)
                        .get(initializer.ident.symbol) else {
                            self.gen_expr(&initializer.expr, None);
                            if self.builder.current_block_diverges() {
                                return Value::Void;
                            }
                            continue;
                        };

                    let field_ptr = self
                        .builder
                        .build_struct_gep(struct_ty, memory, field_index, "")
                        .unwrap();

                    let field_value = self.gen_expr(&initializer.expr, Some(field_ptr));
                    if self.builder.current_block_diverges() {
                        return Value::Void;
                    }

                    match field_value {
                        Value::Fn(fn_value) => {
                            self.builder.build_store(
                                field_ptr,
                                fn_value.as_global_value().as_pointer_value(),
                            );
                        }
                        Value::Register(value) => {
                            self.builder.build_store(field_ptr, value);
                        }
                        Value::Memory(memory) if !initializer.expr.uses_stack_memory() => {
                            let field_ty = self.tys[initializer.expr.expr_id().ty_id].unwrap();
                            let field_value = self.builder.build_load(field_ty, memory, "");
                            self.builder.build_store(field_ptr, field_value);
                        }
                        _ => (),
                    }
                }
                _ => {
                    self.gen_expr(&initializer.expr, None);
                    if self.builder.current_block_diverges() {
                        return Value::Void;
                    }
                }
            }
        }

        memory.map(Value::Memory).unwrap_or(Value::Void)
    }
}
