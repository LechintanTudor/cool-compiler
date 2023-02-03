use crate::lexer::{op, sep, Token, TokenKind};
use crate::parser::{Expr, ParseResult, Parser, UnexpectedToken};
use crate::symbol::{kw, Symbol};
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
            kw::MUT => {
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
                    expected: &[kw::MUT],
                })?;
            }
        };

        self.bump_expect(&[sep::COLON])?;
        self.bump_expect(&[op::EQ])?;

        let expr = self.parse_expr()?;
        let semi = self.bump_expect(&[sep::SEMI])?;

        let span = Span::from_start_and_end_spans(start_token.span, semi.span);

        Ok(DeclStmt {
            span,
            is_mutable,
            ident_span,
            ident,
            expr,
        })
    }
}
