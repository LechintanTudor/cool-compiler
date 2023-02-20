use crate::item::Item;
use crate::{ParseResult, Parser, UnexpectedToken};
use cool_lexer::symbols::Symbol;
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ItemDecl {
    pub span: Span,
    pub is_exported: bool,
    pub ident_span: Span,
    pub ident: Symbol,
    pub item: Item,
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

        let item = match self.peek().kind {
            tk::KW_MODULE => Item::Module(self.parse_module_item()?),
            tk::KW_FN => Item::Fn(self.parse_fn_item()?),
            _ => {
                return Err(UnexpectedToken {
                    found: self.peek(),
                    expected: &[tk::KW_MODULE, tk::KW_FN],
                })?
            }
        };

        let end_token = self.bump_expect(&[tk::SEMICOLON])?;

        Ok(ItemDecl {
            span: start_token.span.to(end_token.span),
            is_exported,
            ident_span,
            ident,
            item,
        })
    }
}
