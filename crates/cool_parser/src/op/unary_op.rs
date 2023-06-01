use crate::{ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UnaryOpKind {
    Minus,
    Not,
    Addr { is_mutable: bool },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UnaryOp {
    pub span: Span,
    pub kind: UnaryOpKind,
}

impl Section for UnaryOp {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_unary_op(&mut self) -> ParseResult<UnaryOp> {
        let start_token = self.bump();

        let unary_op = match start_token.kind {
            tk::MINUS => {
                UnaryOp {
                    span: start_token.span,
                    kind: UnaryOpKind::Minus,
                }
            }
            tk::NOT => {
                UnaryOp {
                    span: start_token.span,
                    kind: UnaryOpKind::Not,
                }
            }
            tk::AND => {
                match self.bump_if_eq(tk::KW_MUT) {
                    Some(end_token) => {
                        UnaryOp {
                            span: start_token.span.to(end_token.span),
                            kind: UnaryOpKind::Addr { is_mutable: true },
                        }
                    }
                    None => {
                        UnaryOp {
                            span: start_token.span,
                            kind: UnaryOpKind::Addr { is_mutable: false },
                        }
                    }
                }
            }
            _ => self.error(start_token, &[tk::MINUS, tk::NOT, tk::AND])?,
        };

        Ok(unary_op)
    }
}
