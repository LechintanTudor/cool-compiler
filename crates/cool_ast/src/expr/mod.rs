mod block_expr;
mod fn_call_expr;
mod ident_expr;
mod literal_expr;

pub use self::block_expr::*;
pub use self::fn_call_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
use crate::{AstGenerator, AstResult};
use cool_parser::Expr;
use cool_resolve::{ExprId, FrameId, TyId};
use paste::paste;

macro_rules! define_expr_ast {
    { $($Variant:ident,)+ } => {
        paste! {
            #[derive(Clone, Debug)]
            pub enum ExprAst {
                $($Variant([<$Variant ExprAst>]),)+   
            }

            impl ExprAst {
                pub fn id(&self) -> ExprId {
                    match self {
                        $(Self::$Variant(e) => e.expr_id,)+
                    }
                }
            }
            
            $(
                impl From<[<$Variant ExprAst>]> for ExprAst {
                    #[inline]
                    fn from(e: [<$Variant ExprAst>]) -> Self {
                        Self::$Variant(e)
                    }
                }
            )+
        }
    };
}

define_expr_ast! {
    Block,
    FnCall,
    Ident,
    Literal,
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
            Expr::FnCall(e) => self.gen_fn_call_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Ident(e) => self.gen_ident_expr(frame_id, expected_ty_id, e)?.into(),
            Expr::Literal(e) => self.gen_literal_expr(expected_ty_id, e)?.into(),
            _ => todo!(),
        };

        Ok(expr)
    }
}
