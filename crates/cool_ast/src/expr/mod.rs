mod block_expr;
mod ident_expr;
mod literal_expr;

pub use self::block_expr::*;
pub use self::ident_expr::*;
pub use self::literal_expr::*;
use crate::AstGenerator;

#[derive(Clone, Debug)]
pub enum ExprAst {
    Block(BlockExprAst),
    Ident(IdentExprAst),
    Literal(LiteralExprAst),
}

impl AstGenerator<'_> {}
