mod access_expr;
mod binary_expr;
mod block_expr;
mod cond_expr;
mod deref_expr;
mod fn_call_expr;
mod ident_expr;
mod literal_expr;
mod unary_expr;
mod while_expr;

pub use self::access_expr::*;
pub use self::binary_expr::*;
pub use self::block_expr::*;
pub use self::cond_expr::*;
pub use self::deref_expr::*;
pub use self::fn_call_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
pub use self::unary_expr::*;
pub use self::while_expr::*;
use crate::{AstGenerator, AstResult, ModuleUsedAsExpr};
use cool_parser::Expr;
use cool_resolve::{ExprId, FrameId, TyId};
use derive_more::From;
use paste::paste;

macro_rules! define_expr_ast {
    { $($Variant:ident,)+ } => {
        paste! {
            #[derive(Clone, From, Debug)]
            pub enum ExprAst {
                $($Variant([<$Variant ExprAst>]),)+
            }
        }

        impl ExprAst {
            pub fn id(&self) -> ExprId {
                match self {
                    $(Self::$Variant(e) => e.expr_id,)+
                }
            }
        }
    };
}

define_expr_ast! {
    Binary,
    Binding,
    Block,
    Cond,
    Deref,
    FnCall,
    Literal,
    Module,
    Unary,
    While,
}

impl ExprAst {
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
            Expr::Binary(e) => self.gen_binary_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Block(e) => self.gen_block_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Cond(e) => self.gen_cond_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Deref(e) => self.gen_deref_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::FnCall(e) => self.gen_fn_call_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Ident(e) => self.gen_ident_expr(frame_id, expected_ty_id, e)?,
            Expr::Literal(e) => self.gen_literal_expr(expected_ty_id, e)?.into(),
            Expr::Paren(e) => self.gen_expr(frame_id, expected_ty_id, &e.inner)?,
            Expr::Unary(e) => self.gen_unary_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::While(e) => self.gen_while_expr(frame_id, expected_ty_id, e)?.into(),
            _ => todo!(),
        };

        Ok(expr)
    }
}
