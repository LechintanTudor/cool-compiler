use crate::{ParseResult, ParseTree, Parser};
use cool_collections::SmallString;
use cool_lexer::symbols::Symbol;
use cool_lexer::tokens::{tk, Literal, LiteralKind, TokenKind};
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct LiteralExpr {
    pub span: Span,
    pub prefix: Option<Symbol>,
    pub literal: Literal,
}

impl ParseTree for LiteralExpr {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_literal_expr(&mut self) -> ParseResult<LiteralExpr> {
        let start_token = self.bump();

        let (prefix, literal, end_token) = match start_token.kind {
            TokenKind::Prefix(symbol) => {
                let next_token = self.bump();

                let TokenKind::Literal(literal) = next_token.kind else {
                    return self.error(next_token, &[tk::ANY_LITERAL]);
                };

                (Some(symbol), literal, next_token)
            }
            TokenKind::Literal(literal) => {
                if literal.kind.is_plain_int() && self.peek_any().kind == tk::DOT {
                    let mut buffer = SmallString::from(literal.symbol.as_str());

                    let dot_token = self.bump_expect(&tk::DOT)?;
                    buffer.push('.');

                    let end_token = if let TokenKind::Literal(literal) = self.peek_any().kind {
                        if literal.kind.is_int() {
                            let end_token = self.bump();
                            buffer.push_str(literal.symbol.as_str());
                            end_token
                        } else {
                            dot_token
                        }
                    } else {
                        dot_token
                    };

                    let literal = Literal {
                        kind: LiteralKind::Decimal,
                        symbol: Symbol::insert(&buffer),
                    };

                    (None, literal, end_token)
                } else {
                    (None, literal, start_token)
                }
            }
            _ => return self.error(start_token, &[tk::ANY_IDENT, tk::ANY_LITERAL]),
        };

        Ok(LiteralExpr {
            span: start_token.span.to(end_token.span),
            prefix,
            literal,
        })
    }
}
