use crate::path::SymbolPath;
use crate::{ParseResult, ParseTree, Parser, UnexpectedToken};
use cool_lexer::tokens::{tk, TokenKind};
use cool_span::Span;
use std::fmt;

#[derive(Clone)]
pub enum Ty {
    Path(SymbolPath),
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

impl fmt::Debug for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Path(path) => fmt::Debug::fmt(path, f),
            Self::Tuple(tuple) => fmt::Debug::fmt(tuple, f),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TupleTy {
    pub span: Span,
    pub elems: Vec<Ty>,
}

impl ParseTree for TupleTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_ty(&mut self) -> ParseResult<Ty> {
        let start_token = self.peek();

        Ok(match start_token.kind {
            tk::OPEN_PAREN => Ty::Tuple(self.parse_tuple_ty()?),
            TokenKind::Ident(_) => Ty::Path(self.parse_import_path()?),
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
            elems: elements,
        })
    }
}
