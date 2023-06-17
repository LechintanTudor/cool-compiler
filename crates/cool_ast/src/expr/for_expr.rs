use crate::{AstGenerator, AstResult, BlockExprAst, DeclStmtAst, ExprAst, StmtAst};
use cool_parser::{ExprOrStmt, ForExpr};
use cool_resolve::{ExprId, FrameId, ResolveExpr, TyId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ForExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub decl: Box<DeclStmtAst>,
    pub cond: Box<ExprAst>,
    pub after: Box<StmtAst>,
    pub body: Box<BlockExprAst>,
}

impl Section for ForExprAst {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl AstGenerator<'_> {
    pub fn gen_for_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &ForExpr,
    ) -> AstResult<ForExprAst> {
        let decl = self.gen_decl_stmt(frame_id, &expr.decl)?;
        self.resolve.make_binding_mutable(decl.binding_id);
        let frame_id = decl.frame_id;

        let cond = self.gen_expr(frame_id, self.tys().bool, &expr.cond)?;

        let after: StmtAst = match expr.after.as_ref() {
            ExprOrStmt::Expr(expr) => self.gen_expr(frame_id, self.tys().infer, expr)?.into(),
            ExprOrStmt::Stmt(stmt) => self.gen_stmt_kind(frame_id, stmt)?,
        };

        let body = self.gen_block_expr(frame_id, self.tys().unit, &expr.body)?;
        let ty_id = self.resolve_direct_ty_id(expr.span(), self.tys().unit, expected_ty_id)?;

        Ok(ForExprAst {
            span: expr.span,
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            decl: Box::new(decl),
            cond: Box::new(cond),
            after: Box::new(after),
            body: Box::new(body),
        })
    }
}
