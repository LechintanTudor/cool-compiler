use crate::error::{ParseResult, UnexpectedToken};
use crate::item::FnItem;
use crate::parser::Parser;
use cool_lexer::symbols::Symbol;
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ItemDecl {
    pub span: Span,
    pub is_exported: bool,
    pub ident_span: Span,
    pub ident: Symbol,
    pub expr: FnItem,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_item_decl(&mut self) -> ParseResult<ItemDecl> {
        let start_token = self.bump();

        let (is_exported, ident_span, ident) = match start_token.kind {
            tk::KW_EXPORT => {
                let next_token = self.bump();

                match next_token.kind {
                    TokenKind::Ident(ident) => (true, next_token.span, ident),
                    _ => {
                        return Err(UnexpectedToken {
                            found: next_token,
                            expected: &[tk::ANY_IDENT],
                        })?;
                    }
                }
            }
            TokenKind::Ident(ident) => (false, start_token.span, ident),
            _ => {
                return Err(UnexpectedToken {
                    found: start_token,
                    expected: &[tk::KW_EXPORT, tk::ANY_IDENT],
                })?;
            }
        };

        self.bump_expect(&[tk::COLON])?;
        self.bump_expect(&[tk::COLON])?;

        let expr = self.parse_fn_item()?;

        let end_token = self.bump_expect(&[tk::SEMICOLON])?;

        Ok(ItemDecl {
            span: start_token.span.to(end_token.span),
            is_exported,
            ident_span,
            ident,
            expr,
        })
    }
}
