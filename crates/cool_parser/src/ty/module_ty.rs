use crate::{ParseResult, Parser};
use cool_lexer::tk;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct ModuleTy {
    pub span: Span,
}

impl Section for ModuleTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_module_ty(&mut self) -> ParseResult<ModuleTy> {
        Ok(ModuleTy {
            span: self.bump_expect(&tk::KW_MODULE)?.span,
        })
    }
}
