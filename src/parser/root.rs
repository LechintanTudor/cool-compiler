use crate::lexer::Token;
use crate::parser::{Parser, StmtAst};

#[derive(Clone, Debug)]
pub struct RootAst {
    pub stmts: Vec<StmtAst>,
}

impl Parser<'_> {
    pub fn parse_root(&mut self) -> anyhow::Result<RootAst> {
        let mut stmts = Vec::new();

        while !self.peek_eq(Token::Eof) {
            stmts.push(self.parse_stmt()?);
        }

        Ok(RootAst { stmts })
    }
}
