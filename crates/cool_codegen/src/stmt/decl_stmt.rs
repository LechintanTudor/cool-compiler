use crate::{CodeGenerator, Value};
use cool_ast::DeclStmtAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_decl_stmt(&mut self, decl: &DeclStmtAst) {
        let binding = self.resolve[decl.binding_id];

        let value = match self.gen_expr(&decl.expr) {
            Value::Memory(memory) => {
                memory.pointer.set_name(binding.symbol.as_str());
                Value::Memory(memory)
            }
            Value::Register(value) => {
                let ptr = self.util_gen_named_init(value, binding.symbol.as_str());
                Value::memory(ptr, value.get_type())
            }
            value => value,
        };

        self.bindings.insert(decl.binding_id, value);
    }
}
