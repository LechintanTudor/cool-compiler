mod access_expr;
mod array_expr;
mod binary_expr;
mod block_expr;
mod cond_expr;
mod deref_expr;
mod fn_call_expr;
mod for_expr;
mod ident_expr;
mod index_expr;
mod literal_expr;
mod struct_expr;
mod tuple_expr;
mod unary_expr;
mod while_expr;

pub use self::access_expr::*;
pub use self::array_expr::*;
pub use self::binary_expr::*;
pub use self::block_expr::*;
pub use self::cond_expr::*;
pub use self::deref_expr::*;
pub use self::fn_call_expr::*;
pub use self::for_expr::*;
pub use self::ident_expr::*;
pub use self::index_expr::*;
pub use self::literal_expr::*;
pub use self::struct_expr::*;
pub use self::tuple_expr::*;
pub use self::unary_expr::*;
pub use self::while_expr::*;
use crate::{AstError, AstGenerator, AstResult};
use cool_parser::{Expr, ParenExpr};
use cool_resolve::{ExprId, FrameId, TyId};
use cool_span::{Section, Span};
use derive_more::From;
use paste::paste;

macro_rules! define_expr_ast {
    { $($Variant:ident,)+ } => {
        paste! {
            #[derive(Clone, From, Debug)]
            pub enum ExprAst {
                $(
                    $Variant([<$Variant ExprAst>]),
                )+
            }

            impl ExprAst {
                pub fn expr_id(&self) -> ExprId {
                    match self {
                        $(
                            Self::$Variant(e) => e.expr_id,
                        )+
                    }
                }

                $(
                    #[inline]
                    pub fn [<as_ $Variant:snake:lower>](&self) -> Option<&[<$Variant ExprAst>]> {
                        match self {
                            Self::$Variant(e) => Some(e),
                            _ => None,
                        }
                    }

                    #[inline]
                    pub fn [<is_ $Variant:snake:lower>](&self) -> bool {
                        matches!(self, Self::$Variant(_))
                    }
                )+
            }

            impl Section for ExprAst {
                fn span(&self) -> Span {
                    match self {
                        $(
                            Self::$Variant(e) => e.span(),
                        )+
                    }
                }
            }
        }
    };
}

define_expr_ast! {
    Access,
    Array,
    ArrayLen,
    ArrayRepeat,
    Binary,
    Binding,
    Block,
    Cond,
    Deref,
    FnCall,
    For,
    Index,
    Literal,
    Module,
    Struct,
    Tuple,
    Ty,
    Unary,
    While,
}

impl ExprAst {
    #[inline]
    pub fn is_aggregate(&self) -> bool {
        matches!(
            self,
            Self::Array(_) | Self::ArrayRepeat(_) | Self::Struct(_),
        )
    }

    #[inline]
    pub fn ensure_not_module(self) -> AstResult<Self> {
        match self {
            Self::Module(_) => Err(AstError::ModuleUsedAsExpr),
            _ => Ok(self),
        }
    }
}

macro_rules! impl_gen_expr {
    { $($Variant:ident,)+ } => {
        impl AstGenerator<'_> {
            pub fn gen_expr(
                &mut self,
                frame_id: FrameId,
                expected_ty_id: TyId,
                expr: &Expr,
            ) -> AstResult<ExprAst> {
                paste! {
                    let expr: ExprAst = match expr {
                        $(
                            Expr::$Variant(expr) => self.[<gen_ $Variant:snake:lower _expr>](
                                frame_id,
                                expected_ty_id,
                                expr,
                            )?.into(),
                        )+
                        expr => todo!("ast generation not yet implemented for {:?}", expr),
                    };
                }

                Ok(expr)
            }
        }
    };
}

impl_gen_expr! {
    Access,
    Array,
    ArrayRepeat,
    Binary,
    Block,
    Cond,
    Deref,
    FnCall,
    For,
    Ident,
    Index,
    Literal,
    Paren,
    Struct,
    Tuple,
    Unary,
    While,
}

impl AstGenerator<'_> {
    #[inline]
    pub fn gen_paren_expr(
        &mut self,
        frame_id: FrameId,
        expected_ty_id: TyId,
        expr: &ParenExpr,
    ) -> AstResult<ExprAst> {
        self.gen_expr(frame_id, expected_ty_id, &expr.inner)
    }
}
