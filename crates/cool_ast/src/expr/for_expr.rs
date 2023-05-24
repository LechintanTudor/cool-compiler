use crate::{AstGenerator, AstResult, BlockExprAst, DeclStmtAst, ExprAst, StmtAst};
use cool_parser::{BareBlockElem, ForExpr};
use cool_resolve::{tys, ExprId, FrameId, ResolveExpr, TyId};

#[derive(Clone, Debug)]
pub struct ForExprAst {
    pub expr_id: ExprId,
    pub decl: Box<DeclStmtAst>,
    pub cond: Box<ExprAst>,
    pub after: Box<StmtAst>,
    pub body: Box<BlockExprAst>,
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

        let cond = self.gen_expr(frame_id, tys::BOOL, &expr.cond)?;

        let after: StmtAst = match expr.after.as_ref() {
            BareBlockElem::Expr(expr) => self.gen_expr(frame_id, tys::INFER, expr)?.into(),
            BareBlockElem::Stmt(stmt) => self.gen_stmt_kind(frame_id, stmt)?,
        };

        let body = self.gen_block_expr(frame_id, tys::UNIT, &expr.body)?;

        let ty_id = self
            .resolve
            .resolve_direct_ty_id(tys::UNIT, expected_ty_id)?;

        Ok(ForExprAst {
            expr_id: self.resolve.add_expr(ResolveExpr::rvalue(ty_id)),
            decl: Box::new(decl),
            cond: Box::new(cond),
            after: Box::new(after),
            body: Box::new(body),
        })
    }
}
