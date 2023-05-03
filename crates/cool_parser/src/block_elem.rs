use crate::expr::Expr;
use crate::stmt::{ExprStmt, Stmt};
use crate::{AssignOp, ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

macro_rules! define_block_elem {
    { $($variant:ident,)+ } => {
        #[derive(Clone)]
        pub enum BlockElem {
            $($variant($variant),)+
        }

        impl ParseTree for BlockElem {
            fn span(&self) -> Span {
                match self {
                    $(Self::$variant(e) => e.span(),)+
                }
            }
        }

        impl std::fmt::Debug for BlockElem {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Self::$variant(s) => std::fmt::Debug::fmt(s, f),)+
                }
            }
        }

        $(
            impl From<$variant> for BlockElem {
                fn from(elem: $variant) -> Self {
                    Self::$variant(elem)
                }
            }
        )+
    };
}

define_block_elem! {
    Expr,
    Stmt,
}

impl Parser<'_> {
    pub fn parse_block_elem(&mut self) -> ParseResult<BlockElem> {
        match self.peek().kind {
            tk::KW_MUT => {
                let stmt = self.parse_decl_stmt()?;
                Ok(BlockElem::Stmt(stmt.into()))
            }
            _ => self.parse_expr_or_decl_or_assign(),
        }
    }

    fn parse_expr_or_decl_or_assign(&mut self) -> ParseResult<BlockElem> {
        let expr = self.parse_expr()?;

        if let Expr::Ident(ident_expr) = &expr {
            if self.peek().kind == tk::COLON {
                let pattern = ident_expr.ident.into();
                let stmt = self.continue_parse_decl(pattern)?;
                return Ok(BlockElem::Stmt(stmt.into()));
            }
        }

        let elem = match self.peek().kind {
            tk::SEMICOLON => {
                let semicolon = self.bump_expect(&tk::SEMICOLON)?;
                BlockElem::Stmt(Stmt::Expr(ExprStmt {
                    span: expr.span().to(semicolon.span),
                    expr: expr.into(),
                }))
            }
            token => {
                match AssignOp::from_token_kind(token) {
                    Some(assign_op) => {
                        self.bump();
                        let stmt = self.continue_parse_assign(expr, assign_op)?;
                        BlockElem::Stmt(stmt.into())
                    }
                    None => BlockElem::Expr(expr),
                }
            }
        };

        Ok(elem)
    }
}
