mod block_expr;
mod fn_call_expr;
mod fn_expr;
mod ident_expr;
mod literal_expr;
mod paren_expr;
mod tuple_expr;

pub use self::block_expr::*;
pub use self::fn_call_expr::*;
pub use self::fn_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
pub use self::paren_expr::*;
use crate::{AstGenerator, ResolveAst, SemanticResult};
use cool_parser::Expr;
use cool_resolve::expr_ty::ExprId;
use cool_resolve::resolve::ScopeId;
use cool_resolve::ty::TyId;
use paste::paste;

pub trait GenericExprAst {
    fn id(&self) -> ExprId;
}

macro_rules! define_expr_ast {
    { $($Variant:ident,)+ } => {
        paste! {
            #[derive(Clone, Debug)]
            pub enum ExprAst {
                $($Variant([<$Variant ExprAst>]),)+
            }
        }

        impl GenericExprAst for ExprAst {
            fn id(&self) -> ExprId {
                match self {
                    $(Self::$Variant(e) => e.id(),)+
                }
            }
        }

        impl ResolveAst for ExprAst {
            fn resolve(&self, ast: &mut AstGenerator, expected_ty: TyId) -> SemanticResult<TyId> {
                match self {
                    $(Self::$Variant(e) => e.resolve(ast, expected_ty),)+
                }
            }
        }

        paste! {
            $(
                impl From<[<$Variant ExprAst>]> for ExprAst {
                    #[inline]
                    fn from(expr: [<$Variant ExprAst>]) -> Self {
                        Self::$Variant(expr)
                    }
                }
            )+
        }
    };
}

define_expr_ast! {
    Block,
    Fn,
    FnCall,
    Ident,
    Literal,
    Paren,
}

impl AstGenerator<'_> {
    pub fn gen_expr(&mut self, scope_id: ScopeId, expr: &Expr) -> ExprAst {
        match expr {
            Expr::Block(e) => ExprAst::Block(self.gen_block_expr(scope_id, e)),
            Expr::Ident(e) => ExprAst::Ident(self.gen_ident_expr(scope_id, e)),
            Expr::Literal(e) => ExprAst::Literal(self.gen_literal_expr(e)),
            Expr::Paren(e) => ExprAst::Paren(self.gen_paren_expr(scope_id, e)),
            Expr::FnCall(e) => ExprAst::FnCall(self.gen_fn_call_expr(scope_id, e)),
            _ => todo!(),
        }
    }
}
