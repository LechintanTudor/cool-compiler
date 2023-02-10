use crate::lexer::{tk, Token, TokenKind};
use crate::parser::{Expr, ParseResult, Parser, UnexpectedToken};
use crate::symbol::Symbol;
use crate::utils::Span;

#[derive(Clone, Debug)]
pub struct DeclStmt {
    pub span: Span,
    pub is_mutable: bool,
    pub ident_span: Span,
    pub ident: Symbol,
    pub expr: Expr,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_decl_stmt(&mut self) -> ParseResult<DeclStmt> {
        let start_token = self.bump();

        let (is_mutable, ident_span, ident) = match start_token.kind {
            tk::KW_MUT => {
                let next_token = self.bump();

                match next_token.kind {
                    TokenKind::Ident(ident) => (true, next_token.span, ident),
                    _ => {
                        return Err(UnexpectedToken {
                            found: next_token,
                            expected: &[],
                        })?;
                    }
                }
            }
            TokenKind::Ident(ident) => (false, start_token.span, ident),
            _ => {
                return Err(UnexpectedToken {
                    found: start_token,
                    expected: &[tk::KW_MUT],
                })?;
            }
        };

        self.bump_expect(&[tk::COLON])?;
        self.bump_expect(&[tk::EQ])?;

        let expr = self.parse_expr()?;
        let semi = self.bump_expect(&[tk::SEMICOLON])?;

        Ok(DeclStmt {
            span: start_token.span.to(semi.span),
            is_mutable,
            ident_span,
            ident,
            expr,
        })
    }
}
