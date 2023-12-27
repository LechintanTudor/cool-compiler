use crate::{ParseResult, Parser};
use cool_lexer::{tk, LiteralKind, Symbol, TokenKind};

#[derive(Clone, Copy, Debug)]
pub enum FnAbi {
    Implicit,
    Explicit(Option<Symbol>),
}

impl FnAbi {
    #[inline]
    #[must_use]
    pub fn is_explicit(&self) -> bool {
        matches!(self, Self::Explicit(_))
    }
}

impl Parser<'_> {
    pub fn parse_fn_abi(&mut self) -> ParseResult<FnAbi> {
        if self.peek().kind != tk::kw_extern {
            return Ok(FnAbi::Implicit);
        }

        self.bump_expect(&tk::kw_extern)?;
        let symbol = self.parse_str_literal();
        Ok(FnAbi::Explicit(symbol))
    }

    fn parse_str_literal(&mut self) -> Option<Symbol> {
        let token = self.peek();

        let TokenKind::Literal(literal) = token.kind else {
            return None;
        };

        if literal.kind != LiteralKind::Str {
            return None;
        };

        self.bump();
        Some(literal.value)
    }
}
