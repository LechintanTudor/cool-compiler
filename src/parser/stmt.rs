use crate::lexer::{Operator, Separator};
use crate::parser::{ExprAst, Parser, PatAst, TyAst};

#[derive(Clone, Debug)]
pub enum StmtAst {
    Decl(DeclStmtAst),
}

#[derive(Clone, Debug)]
pub struct DeclStmtAst {
    pub pat: PatAst,
    pub ty: Option<TyAst>,
    pub expr: ExprAst,
}

impl Parser<'_> {
    pub fn parse_stmt(&mut self) -> anyhow::Result<StmtAst> {
        let pat = self.parse_pat()?;

        if !self.consume_if_eq(Separator::Colon) {
            panic!("missing ':' in declaration statement");
        }

        let ty = if self.peek_eq(Operator::Equal) {
            None
        } else {
            Some(self.parse_ty()?)
        };

        if !self.consume_if_eq(Operator::Equal) {
            panic!("missing '=' in declaration statement");
        }

        let expr = self.parse_expr()?;

        if !self.consume_if_eq(Separator::Semicolon) {
            panic!("missing ';' at the end of statement");
        }

        Ok(StmtAst::Decl(DeclStmtAst { pat, ty, expr }))
    }
}
