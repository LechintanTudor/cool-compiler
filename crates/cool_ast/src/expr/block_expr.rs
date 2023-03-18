use crate::expr::GenericExprAst;
use crate::stmt::StmtAst;
use crate::{AstGenerator, BlockElemAst};
use cool_parser::{BlockElem, BlockExpr, Stmt};
use cool_resolve::binding::{BindingTable, FrameId};
use cool_resolve::item::ItemId;
use cool_resolve::ty::{tys, TyId};

#[derive(Clone, Debug)]
pub struct BlockExprAst {
    pub frame_id: FrameId,
    pub elems: Vec<BlockElemAst>,
}

impl GenericExprAst for BlockExprAst {
    fn ty_id(&self, bindings: &BindingTable) -> Option<TyId> {
        match self.elems.last() {
            Some(BlockElemAst::Expr(expr)) => expr.ty_id(bindings),
            _ => Some(tys::UNIT),
        }
    }
}

impl AstGenerator<'_> {
    pub fn generate_block_expr(
        &mut self,
        module_id: ItemId,
        parent_id: Option<FrameId>,
        expr: &BlockExpr,
    ) -> BlockExprAst {
        let frame_id = self.bindings.add_frame(module_id, parent_id);
        let mut last_frame_id = frame_id;

        let mut elems = Vec::<BlockElemAst>::new();

        for elem in expr.elems.iter() {
            let elem: BlockElemAst = match elem {
                BlockElem::Stmt(Stmt::Decl(decl)) => {
                    let decl_ast = self.generate_decl_stmt(module_id, Some(last_frame_id), decl);
                    last_frame_id = decl_ast.frame_id;
                    BlockElemAst::Stmt(StmtAst::Decl(decl_ast))
                }
                BlockElem::Expr(expr) => {
                    let expr_ast = self.generate_expr(module_id, Some(last_frame_id), expr);
                    BlockElemAst::Expr(expr_ast)
                }
                _ => todo!(),
            };

            elems.push(elem);
        }

        BlockExprAst { frame_id, elems }
    }
}
