use crate::{BuilderExt, CodeGenerator, Value};
use cool_ast::TupleExprAst;
use cool_lexer::Symbol;
use inkwell::values::PointerValue;

impl<'a> CodeGenerator<'a> {
    pub fn gen_tuple_expr(
        &mut self,
        expr: &TupleExprAst,
        memory: Option<PointerValue<'a>>,
    ) -> Value<'a> {
        let tuple_ty_id = expr.expr_id.ty_id;
        let tuple_ty = self.tys[tuple_ty_id];
        let memory = memory.or_else(|| tuple_ty.map(|ty| self.util_gen_alloca(ty)));

        for (i, expr) in expr.elems.iter().enumerate() {
            match (tuple_ty, memory) {
                (Some(struct_ty), Some(memory)) => {
                    let Some(field_index) = self
                        .tys
                        .get_field_map(tuple_ty_id)
                        .get(Symbol::insert_u32(i as _)) else {
                            self.gen_expr(expr, None);
                            if self.builder.current_block_diverges() {
                                return Value::Void;
                            }
                            continue;
                        };

                    let field_ptr = self
                        .builder
                        .build_struct_gep(struct_ty, memory, field_index, "")
                        .unwrap();

                    let field_value = self.gen_expr(expr, Some(field_ptr));
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
                        _ => (),
                    }
                }
                _ => {
                    self.gen_expr(expr, None);
                    if self.builder.current_block_diverges() {
                        return Value::Void;
                    }
                }
            }
        }

        memory.map(Value::Memory).unwrap_or(Value::Void)
    }
}
