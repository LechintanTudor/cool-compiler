use crate::{ParseResult, ParseTree, Parser};
use cool_lexer::tokens::tk;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct ModuleTy {
    pub span: Span,
}

impl ParseTree for ModuleTy {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl Parser<'_> {
    pub fn parse_module_ty(&mut self) -> ParseResult<ModuleTy> {
        let module_kw = self.bump_expect(&tk::KW_MODULE)?;

        Ok(ModuleTy {
            span: module_kw.span,
        })
    }
}
