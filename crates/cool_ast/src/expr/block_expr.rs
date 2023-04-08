use crate::expr::GenericExprAst;
use crate::stmt::StmtAst;
use crate::{AstGenerator, AstResult, BlockElemAst, ResolveAst, TyMismatch};
use cool_parser::{BlockElem, BlockExpr, Stmt};
use cool_resolve::{tys, ExprId, FrameId, ScopeId, TyId};

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

impl ResolveAst for BlockExprAst {
    fn resolve_exprs(&self, ast: &mut AstGenerator, expected_ty: TyId) -> AstResult<TyId> {
        let expr_ty = match self.elems.split_last() {
            Some((last, others)) => {
                for other in others {
                    other.resolve_exprs(ast, tys::INFERRED)?;
                }

                last.resolve_exprs(ast, expected_ty)?
            }
            None => tys::UNIT
                .resolve_non_inferred(expected_ty)
                .ok_or(TyMismatch {
                    found_ty: tys::UNIT,
                    expected_ty,
                })?,
        };

        ast.resolve.set_expr_ty(self.id, expr_ty);
        Ok(expr_ty)
    }
}

impl AstGenerator<'_> {
    pub fn gen_block_expr(&mut self, scope_id: ScopeId, expr: &BlockExpr) -> BlockExprAst {
        let id = self.resolve.add_expr();

        let frame_id = self.resolve.add_frame(scope_id);
        let mut current_frame_id = frame_id;

        let mut elems = Vec::<BlockElemAst>::new();

        for elem in expr.elems.iter() {
            let elem: BlockElemAst = match elem {
                BlockElem::Stmt(stmt) => match stmt {
                    Stmt::Decl(decl) => {
                        let decl_ast = self.gen_decl_stmt(current_frame_id.into(), decl);
                        current_frame_id = decl_ast.frame_id;
                        BlockElemAst::Stmt(StmtAst::Decl(decl_ast))
                    }
                    Stmt::Expr(expr) => {
                        let expr_stmt_ast = self.gen_expr_stmt(current_frame_id.into(), expr);
                        BlockElemAst::Stmt(StmtAst::Expr(expr_stmt_ast))
                    }
                    _ => todo!(),
                },
                BlockElem::Expr(expr) => {
                    let expr_ast = self.gen_expr(current_frame_id.into(), expr);
                    BlockElemAst::Expr(expr_ast)
                }
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
