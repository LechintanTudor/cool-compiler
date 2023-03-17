use crate::expr::GenericExprAst;
use crate::{BlockElemAst, AstGenerator};
use cool_parser::{BlockExpr, BlockElem, Stmt};
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

        let mut elems = Vec::<BlockElemAst>::new();
        
        for elem in expr.elems.iter() {
            match elem {
                BlockElem::Stmt(Stmt::Decl(decl)) => {
                    todo!()
                }
                _ => todo!()
            }
        }

        BlockExprAst { frame_id, elems }
    }
}