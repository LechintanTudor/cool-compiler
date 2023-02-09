use crate::lexer::{op, sep, Token, TokenKind};
use crate::parser::{ParseResult, Parser, UnexpectedToken};
use crate::symbol::Symbol;
use crate::utils::Span;
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
            sep::OPEN_PAREN => Ty::Tuple(self.parse_tuple_ty()?),
            TokenKind::Ident(_) => Ty::Path(self.parse_path_ty()?),
            _ => {
                return Err(UnexpectedToken {
                    found: start_token,
                    expected: &[sep::OPEN_PAREN],
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

        while self.peek().kind == op::DOT {
            self.bump_expect(&[op::DOT])?;
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
        let open_paren = self.bump_expect(&[sep::OPEN_PAREN])?;
        let mut elements = Vec::<Ty>::new();

        let closed_paren = if self.peek().kind == sep::CLOSED_PAREN {
            self.bump()
        } else {
            loop {
                let element = self.parse_ty()?;
                elements.push(element);

                let next_token = self.bump();

                match next_token.kind {
                    sep::COMMA => {
                        if self.peek().kind == sep::CLOSED_PAREN {
                            break self.bump();
                        }
                    }
                    sep::CLOSED_PAREN => {
                        break next_token;
                    }
                    _ => {
                        return Err(UnexpectedToken {
                            found: next_token,
                            expected: &[sep::COMMA, sep::CLOSED_PAREN],
                        })?
                    }
                }
            }
        };

        Ok(TupleTy {
            span: Span::from_start_and_end_spans(open_paren.span, closed_paren.span),
            elements,
        })
    }
}
