use crate::expr::ExprAst;
use crate::stmt::StmtAst;
use crate::AstGenerator;
use cool_parser::BlockElem;
use cool_resolve::resolve::ScopeId;

#[derive(Clone, Debug)]
pub enum BlockElemAst {
    Expr(ExprAst),
    Stmt(StmtAst),
}

impl AstGenerator<'_> {
    pub fn gen_block_elem(&mut self, _scope_id: ScopeId, _block_elem: &BlockElem) -> BlockElemAst {
        todo!()
    }
}
