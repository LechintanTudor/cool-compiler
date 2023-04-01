use crate::expr::GenericExprAst;
use crate::stmt::StmtAst;
use crate::{AstGenerator, BlockElemAst, Unify};
use cool_parser::{BlockElem, BlockExpr, Stmt};
use cool_resolve::expr_ty::{Constraint, ExprId};
use cool_resolve::resolve::{FrameId, ScopeId};
use cool_resolve::ty::tys;

#[derive(Clone, Debug)]
pub struct BlockExprAst {
    pub id: ExprId,
    pub frame_id: FrameId,
    pub elems: Vec<BlockElemAst>,
}

impl GenericExprAst for BlockExprAst {
    #[inline]
    fn id(&self) -> ExprId {
        self.id
    }
}

impl Unify for BlockExprAst {
    fn unify(&self, gen: &mut AstGenerator) {
        for elem in self.elems.iter() {
            elem.unify(gen);
        }

        let rhs_constraint = match self.elems.last() {
            Some(BlockElemAst::Expr(expr)) => Constraint::Expr(expr.id()),
            _ => Constraint::Ty(tys::UNIT),
        };

        gen.unification
            .add_constraint(Constraint::Expr(self.id), rhs_constraint);
    }
}

impl AstGenerator<'_> {
    pub fn gen_block_expr(&mut self, scope_id: ScopeId, expr: &BlockExpr) -> BlockExprAst {
        let id = self.unification.gen_expr();

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

        BlockExprAst {
            id,
            frame_id,
            elems,
        }
    }
}
