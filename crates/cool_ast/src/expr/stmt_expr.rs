use crate::{AstGenerator, AstResult, AstResultExt, ExprAst, LogicError, StmtAst};
use cool_parser::{ExprOrStmt, StmtKind};
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct StmtExprAst {
    pub expr_id: ExprId,
    pub stmt: Box<StmtAst>,
}

impl Section for StmtExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.stmt.span()
    }
}

impl AstGenerator<'_> {
    pub fn gen_stmt_expr_or_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        code: &ExprOrStmt,
    ) -> AstResult<ExprAst> {
        match code {
            ExprOrStmt::Expr(expr) => self.gen_expr(frame_id, expected_ty_id, expr),
            ExprOrStmt::Stmt(stmt) => self.gen_stmt_expr(frame_id, expected_ty_id, stmt),
        }
    }

    pub fn gen_stmt_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        stmt: &StmtKind,
    ) -> AstResult<ExprAst> {
        if let Some(expr) = stmt.as_expr() {
            return self.gen_expr(frame_id, expected_ty_id, expr);
        }

        if !stmt.is_promotable_to_expr() {
            return AstResult::error(stmt.span(), LogicError::StmtNotPromotableToExpr);
        }

        let stmt = self.gen_stmt_kind(frame_id, stmt)?;

        let found_ty_id = if stmt.diverges() {
            self.tys().diverge
        } else {
            self.tys().unit
        };

        self.resolve_expr(
            stmt.span(),
            found_ty_id,
            expected_ty_id,
            |resolve, _, ty_id| {
                StmtExprAst {
                    expr_id: resolve.add_expr(ResolveExpr::rvalue(ty_id)),
                    stmt: Box::new(stmt),
                }
            },
        )
    }
}
