mod access_expr;
mod align_of_expr;
mod array_expr;
mod binary_expr;
mod block_expr;
mod cast_expr;
mod cond_expr;
mod deref_expr;
mod fn_call_expr;
mod ident_expr;
mod index_expr;
mod literal_expr;
mod match_expr;
mod offset_of_expr;
mod range_expr;
mod size_of_expr;
mod stmt_expr;
mod struct_expr;
mod tuple_expr;
mod unary_expr;
mod unit_expr;
mod variant_wrap_expr;

pub use self::access_expr::*;
pub use self::align_of_expr::*;
pub use self::array_expr::*;
pub use self::binary_expr::*;
pub use self::block_expr::*;
pub use self::cast_expr::*;
pub use self::cond_expr::*;
pub use self::deref_expr::*;
pub use self::fn_call_expr::*;
pub use self::ident_expr::*;
pub use self::index_expr::*;
pub use self::literal_expr::*;
pub use self::match_expr::*;
pub use self::offset_of_expr::*;
pub use self::range_expr::*;
pub use self::size_of_expr::*;
pub use self::stmt_expr::*;
pub use self::struct_expr::*;
pub use self::tuple_expr::*;
pub use self::unary_expr::*;
pub use self::unit_expr::*;
pub use self::variant_wrap_expr::*;
use crate::{AstGenerator, AstResult};
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
    AlignOf,
    Array,
    ArrayLen,
    ArrayRepeat,
    Binary,
    Binding,
    Block,
    Cast,
    Cond,
    Deref,
    FnCall,
    Index,
    Literal,
    Match,
    Module,
    OffsetOf,
    Range,
    SizeOf,
    Stmt,
    Struct,
    Tuple,
    Ty,
    Unary,
    Unit,
    VariantWrap,
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
                        expr => todo!("ast generation not yet implemented for {:#?}", expr),
                    };
                }

                Ok(expr)
            }
        }
    };
}

impl_gen_expr! {
    Access,
    AlignOf,
    Array,
    ArrayRepeat,
    Binary,
    Block,
    Cast,
    Cond,
    Deref,
    FnCall,
    Ident,
    Index,
    Literal,
    Match,
    OffsetOf,
    Paren,
    Range,
    SizeOf,
    Struct,
    Tuple,
    Unary,
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
