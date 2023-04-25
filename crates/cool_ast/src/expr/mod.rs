mod block_expr;
mod ident_expr;
mod literal_expr;

pub use self::block_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
use crate::{AstGenerator, AstResult};
use cool_parser::Expr;
use cool_resolve::{FrameId, TyId};

#[derive(Clone, Debug)]
pub enum ExprAst {
    Block(BlockExprAst),
    Ident(IdentExprAst),
    Literal(LiteralExprAst),
}

impl From<BlockExprAst> for ExprAst {
    #[inline]
    fn from(e: BlockExprAst) -> Self {
        Self::Block(e)
    }
}

impl From<IdentExprAst> for ExprAst {
    #[inline]
    fn from(e: IdentExprAst) -> Self {
        Self::Ident(e)
    }
}

impl From<LiteralExprAst> for ExprAst {
    #[inline]
    fn from(e: LiteralExprAst) -> Self {
        Self::Literal(e)
    }
}

impl AstGenerator<'_> {
    pub fn gen_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &Expr,
    ) -> AstResult<ExprAst> {
        let expr: ExprAst = match expr {
            Expr::Block(e) => self.gen_block_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Ident(e) => self.gen_ident_expr(frame_id, e)?.into(),
            Expr::Literal(e) => self.gen_literal_expr(expected_ty_id, e)?.into(),
            _ => todo!(),
        };

        Ok(expr)
    }
}
