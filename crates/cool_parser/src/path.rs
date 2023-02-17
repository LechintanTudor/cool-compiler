use crate::error::{ParseResult, UnexpectedToken};
use crate::parser::Parser;
use crate::ParseTree;
use cool_lexer::symbols::Symbol;
use cool_lexer::tokens::{tk, Token, TokenKind};
use cool_span::Span;
use smallvec::SmallVec;

pub type PathFragmentVec = SmallVec<[PathFragment; 2]>;

#[derive(Clone, Debug)]
pub struct Path {
    pub fragments: PathFragmentVec,
}

impl ParseTree for Path {
    fn span(&self) -> Span {
        match (self.fragments.first(), self.fragments.last()) {
            (Some(first), Some(last)) => first.span_to(last),
            _ => Span::empty(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PathFragment {
    pub span: Span,
    pub ident: Symbol,
}

impl ParseTree for PathFragment {
    fn span(&self) -> Span {
        self.span
    }
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    pub fn parse_path(&mut self) -> ParseResult<Path> {
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
                expected: &[tk::ANY_IDENT],
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
                    expected: &[tk::ANY_IDENT],
                })?;
            }
        }

        Ok(Path { fragments })
    }
}
