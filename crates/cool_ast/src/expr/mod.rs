mod access_expr;
mod array_expr;
mod binary_expr;
mod block_expr;
mod cond_expr;
mod deref_expr;
mod fn_call_expr;
mod ident_expr;
mod literal_expr;
mod subscript_expr;
mod unary_expr;
mod while_expr;

pub use self::access_expr::*;
pub use self::array_expr::*;
pub use self::binary_expr::*;
pub use self::block_expr::*;
pub use self::cond_expr::*;
pub use self::deref_expr::*;
pub use self::fn_call_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
pub use self::subscript_expr::*;
pub use self::unary_expr::*;
pub use self::while_expr::*;
use crate::{AstGenerator, AstResult, ModuleUsedAsExpr};
use cool_parser::Expr;
use cool_resolve::{ExprId, FrameId, TyId};
use derive_more::From;

#[derive(Clone, From, Debug)]
pub enum ExprAst {
    Array(ArrayExprAst),
    Binary(BinaryExprAst),
    Binding(BindingExprAst),
    Block(BlockExprAst),
    Cond(CondExprAst),
    Deref(DerefExprAst),
    FnCall(FnCallExprAst),
    Literal(LiteralExprAst),
    Module(ModuleExprAst),
    Subscript(SubscriptExprAst),
    Unary(UnaryExprAst),
    While(WhileExprAst),
}

impl ExprAst {
    pub fn expr_id(&self) -> ExprId {
        match self {
            Self::Array(e) => e.expr_id,
            Self::Binary(e) => e.expr_id,
            Self::Binding(e) => e.expr_id,
            Self::Block(e) => e.expr_id,
            Self::Cond(e) => e.expr_id,
            Self::Deref(e) => e.expr_id,
            Self::FnCall(e) => e.expr_id,
            Self::Literal(e) => e.expr_id,
            Self::Module(e) => e.expr_id,
            Self::Subscript(e) => e.expr_id,
            Self::Unary(e) => e.expr_id,
            Self::While(e) => e.expr_id,
        }
    }

    #[inline]
    pub fn is_module(&self) -> bool {
        matches!(self, Self::Module(_))
    }

    #[inline]
    pub fn ensure_not_module(self) -> Result<Self, ModuleUsedAsExpr> {
        match self {
            Self::Module(_) => Err(ModuleUsedAsExpr),
            _ => Ok(self),
        }
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
            Expr::Access(e) => self.gen_access_expr(frame_id, expected_ty_id, e)?,
            Expr::Array(e) => self.gen_array_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Binary(e) => self.gen_binary_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Block(e) => self.gen_block_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Cond(e) => self.gen_cond_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Deref(e) => self.gen_deref_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::FnCall(e) => self.gen_fn_call_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Ident(e) => self.gen_ident_expr(frame_id, expected_ty_id, e)?,
            Expr::Literal(e) => self.gen_literal_expr(expected_ty_id, e)?.into(),
            Expr::Paren(e) => self.gen_expr(frame_id, expected_ty_id, &e.inner)?,
            Expr::Unary(e) => self.gen_unary_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Subscript(e) => self.gen_subscript_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::While(e) => self.gen_while_expr(frame_id, expected_ty_id, e)?.into(),
            _ => todo!(),
        };

        Ok(expr)
    }
}
