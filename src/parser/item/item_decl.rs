use crate::lexer::{tk, Token, TokenKind};
use crate::parser::{FnItem, ParseResult, Parser, UnexpectedToken};
use crate::symbol::Symbol;
use crate::utils::Span;

#[derive(Clone, Debug)]
pub struct ItemDecl {
    pub span: Span,
    pub ident: Symbol,
    pub expr: FnItem,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_static_decl(&mut self) -> ParseResult<ItemDecl> {
        let start_token = self.bump();

        let ident = match start_token.kind {
            TokenKind::Ident(ident) => ident,
            _ => {
                return Err(UnexpectedToken {
                    found: start_token,
                    expected: &[],
                })?;
            }
        };

        self.bump_expect(&[tk::COLON])?;
        self.bump_expect(&[tk::COLON])?;

        let expr = self.parse_fn_item()?;

        let end_token = self.bump_expect(&[tk::SEMICOLON])?;

        Ok(ItemDecl {
            span: start_token.span.to(end_token.span),
            ident,
            expr,
        })
    }
}
