use crate::error::{ParseResult, UnexpectedToken};
use crate::parser::Parser;
use cool_lexer::symbols::Symbol;
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;
use smallvec::SmallVec;

pub type PathFragmentVec = SmallVec<[PathFragment; 2]>;

#[derive(Clone, Debug)]
pub enum Ty {
    Path(PathTy),
    Tuple(TupleTy),
}

#[derive(Clone, Debug)]
pub struct PathTy {
    pub fragments: SmallVec<[PathFragment; 2]>,
}

#[derive(Clone, Debug)]
pub struct PathFragment {
    pub span: Span,
    pub ident: Symbol,
}

#[derive(Clone, Debug)]
pub struct TupleTy {
    pub span: Span,
    pub elements: Vec<Ty>,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_ty(&mut self) -> ParseResult<Ty> {
        let start_token = self.peek();

        Ok(match start_token.kind {
            tk::OPEN_PAREN => Ty::Tuple(self.parse_tuple_ty()?),
            TokenKind::Ident(_) => Ty::Path(self.parse_path_ty()?),
            _ => {
                return Err(UnexpectedToken {
                    found: start_token,
                    expected: &[tk::OPEN_PAREN],
                })?;
            }
        })
    }

    pub fn parse_path_ty(&mut self) -> ParseResult<PathTy> {
        let start_token = self.bump();
        let mut fragments = PathFragmentVec::new();

        if let TokenKind::Ident(ident) = start_token.kind {
            fragments.push(PathFragment {
                span: start_token.span,
                ident,
            });
        } else {
            return Err(UnexpectedToken {
                found: start_token,
                expected: &[],
            })?;
        }

        while self.peek().kind == tk::DOT {
            self.bump_expect(&[tk::DOT])?;
            let token = self.bump();

            if let TokenKind::Ident(ident) = token.kind {
                fragments.push(PathFragment {
                    span: token.span,
                    ident,
                });
            } else {
                return Err(UnexpectedToken {
                    found: token,
                    expected: &[],
                })?;
            }
        }

        Ok(PathTy { fragments })
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
