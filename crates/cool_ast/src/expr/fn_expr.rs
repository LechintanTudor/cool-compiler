use crate::{AstGenerator, AstResult, BlockExprAst, ExprAst};
use cool_collections::SmallVec;
use cool_derive::Section;
use cool_parser::FnExpr;
use cool_resolve::{BindingId, ExprId, Scope, TyId};
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct FnExprAst {
    pub span: Span,
    pub expr_id: ExprId,
    pub params: SmallVec<BindingId, 4>,
    pub body: BlockExprAst,
}

impl AstGenerator<'_> {
    pub fn gen_fn_expr<S>(
        &mut self,
        expr: &FnExpr,
        scope: S,
        expected_ty_id: TyId,
    ) -> AstResult<ExprAst>
    where
        S: Into<Scope>,
    {
        todo!()
    }
}
