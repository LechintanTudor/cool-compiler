use crate::CodeGenerator;
use cool_ast::BlockElemAst;
use inkwell::values::AnyValueEnum;

impl<'a> CodeGenerator<'a> {
    pub fn gen_block_elem(&mut self, elem: &BlockElemAst) -> Option<AnyValueEnum<'a>> {
        match elem {
            BlockElemAst::Expr(expr) => self.gen_expr(expr),
            BlockElemAst::Stmt(stmt) => {
                self.gen_stmt(stmt);
                None
            }
        }
    }
}
