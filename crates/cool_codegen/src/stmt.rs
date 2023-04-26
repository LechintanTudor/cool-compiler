use crate::CodeGenerator;
use cool_ast::StmtAst;

impl<'a> CodeGenerator<'a> {
    pub fn gen_stmt(&mut self, stmt: &StmtAst) {
        match stmt {
            StmtAst::Expr(expr) => {
                self.gen_expr(expr);
            }
            _ => (),
        };
    }
}
