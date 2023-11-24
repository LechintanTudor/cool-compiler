mod block_expr;
mod fn_expr;
mod ident_expr;
mod literal_expr;
mod wrap_expr;

pub use self::block_expr::*;
pub use self::fn_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
pub use self::wrap_expr::*;

use crate::{AstGenerator, SpannedAstResult, WithSpan};
use cool_derive::Section;
use cool_parser::Expr;
use cool_resolve::{ExprId, FrameId, ResolveContext, TyId, UnificationMethod};
use cool_span::Span;
use derive_more::From;

#[derive(Clone, Section, From, Debug)]
pub enum ExprAst {
    Block(BlockExprAst),
    Fn(FnExprAst),
    Ident(IdentExprAst),
    Literal(LiteralExprAst),
    Wrap(WrapExprAst),
}

impl ExprAst {
    #[inline]
    #[must_use]
    pub fn expr_id(&self) -> ExprId {
        match self {
            ExprAst::Block(e) => e.expr_id,
            ExprAst::Fn(e) => e.expr_id,
            ExprAst::Ident(e) => e.expr_id,
            ExprAst::Literal(e) => e.expr_id,
            ExprAst::Wrap(e) => e.expr_id,
        }
    }
}

impl AstGenerator<'_> {
    pub fn gen_expr(
        &mut self,
        expr: &Expr,
        frame_id: FrameId,
        expected_ty_id: TyId,
    ) -> SpannedAstResult<ExprAst> {
        let expr = match expr {
            Expr::Block(e) => self.gen_block_expr(e, frame_id, expected_ty_id)?,
            Expr::Fn(e) => {
                let module_id = self.context.get_toplevel_module(frame_id);
                self.gen_fn_expr(e, module_id, expected_ty_id)?
            }
            Expr::Ident(e) => self.gen_ident_expr(e, frame_id, expected_ty_id)?,
            Expr::Literal(e) => self.gen_literal_expr(e, expected_ty_id)?,
            _ => todo!(),
        };

        Ok(expr)
    }

    pub fn gen_tail_expr<E, B>(
        &mut self,
        span: Span,
        found_ty_id: TyId,
        expected_ty_id: TyId,
        expr_builder: B,
    ) -> SpannedAstResult<ExprAst>
    where
        E: Into<ExprAst>,
        B: FnOnce(&mut ResolveContext, Span, TyId) -> E,
    {
        let (ty_id, method) = self
            .context
            .unify_tys(found_ty_id, expected_ty_id)
            .with_span(span)?;

        let expr = match method {
            UnificationMethod::Direct => expr_builder(self.context, span, ty_id).into(),
            UnificationMethod::Wrap => {
                let expr = expr_builder(self.context, span, found_ty_id);

                self.continue_gen_wrap_expr(expr.into(), expected_ty_id)
                    .into()
            }
        };

        Ok(expr)
    }
}
