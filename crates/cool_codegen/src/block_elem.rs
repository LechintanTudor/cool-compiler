use crate::{CodeGenerator, Value};
use cool_ast::BlockElemAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_block_elem(&mut self, elem: &BlockElemAst) -> Value<'a> {
        match elem {
            BlockElemAst::Expr(expr) => self.gen_expr(expr),
            BlockElemAst::Stmt(stmt) => {
                self.gen_stmt(stmt);
                Value::Void
            }
        }
    }
}
