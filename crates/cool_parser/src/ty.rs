use crate::path::ItemPath;
use crate::{ParseResult, ParseTree, Parser, UnexpectedToken};
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub enum Ty {
    Path(ItemPath),
    Tuple(TupleTy),
}

impl ParseTree for Ty {
    fn span(&self) -> Span {
        match self {
            Self::Path(path) => path.span(),
            Self::Tuple(tuple) => tuple.span(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TupleTy {
    pub span: Span,
    pub elements: Vec<Ty>,
}

impl ParseTree for TupleTy {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_ty(&mut self) -> ParseResult<Ty> {
        let start_token = self.peek();

        Ok(match start_token.kind {
            tk::OPEN_PAREN => Ty::Tuple(self.parse_tuple_ty()?),
            TokenKind::Ident(_) => Ty::Path(self.parse_path()?),
            _ => {
                return Err(UnexpectedToken {
                    found: start_token,
                    expected: &[tk::OPEN_PAREN],
                })?;
            }
        })
    }

    pub fn parse_tuple_ty(&mut self) -> ParseResult<TupleTy> {
        let open_paren = self.bump_expect(&[tk::OPEN_PAREN])?;
        let mut elements = Vec::<Ty>::new();

        let closed_paren = if self.peek().kind == tk::CLOSE_PAREN {
            self.bump()
        } else {
            loop {
                let element = self.parse_ty()?;
                elements.push(element);

                let next_token = self.bump();

                match next_token.kind {
                    tk::COMMA => {
                        if self.peek().kind == tk::CLOSE_PAREN {
                            break self.bump();
                        }
                    }
                    tk::CLOSE_PAREN => {
                        break next_token;
                    }
                    _ => {
                        return Err(UnexpectedToken {
                            found: next_token,
                            expected: &[tk::COMMA, tk::CLOSE_PAREN],
                        })?;
                    }
                }
            }
        };

        Ok(TupleTy {
            span: open_paren.span.to(closed_paren.span),
            elements,
        })
    }
}
