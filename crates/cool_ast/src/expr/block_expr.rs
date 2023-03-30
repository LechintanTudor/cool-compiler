use crate::stmt::StmtAst;
use crate::{AstGenerator, BlockElemAst};
use cool_parser::{BlockElem, BlockExpr, Stmt};
use cool_resolve::resolve::{FrameId, ScopeId};

#[derive(Clone, Debug)]
pub struct BlockExprAst {
    pub frame_id: FrameId,
    pub elems: Vec<BlockElemAst>,
}

impl AstGenerator<'_> {
    pub fn gen_block_expr(&mut self, scope_id: ScopeId, expr: &BlockExpr) -> BlockExprAst {
        let frame_id = self.resolve.insert_frame(scope_id);
        let mut current_frame_id = frame_id;

        let mut elems = Vec::<BlockElemAst>::new();

        for elem in expr.elems.iter() {
            let elem: BlockElemAst = match elem {
                BlockElem::Stmt(Stmt::Decl(decl)) => {
                    let decl_ast = self.gen_decl_stmt(current_frame_id.into(), decl);
                    current_frame_id = decl_ast.frame_id;
                    BlockElemAst::Stmt(StmtAst::Decl(decl_ast))
                }
                BlockElem::Expr(expr) => {
                    let expr_ast = self.gen_expr(current_frame_id.into(), expr);
                    BlockElemAst::Expr(expr_ast)
                }
                _ => todo!(),
            };

            elems.push(elem);
        }

        BlockExprAst { frame_id, elems }
    }
}
