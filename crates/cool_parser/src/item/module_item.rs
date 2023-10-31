use crate::{Module, ParseResult, Parser};
use cool_derive::Section;
use cool_lexer::tk;
use cool_span::Span;

#[derive(Clone, Section, Debug)]
pub struct ModuleItem {
    pub span: Span,
    pub kind: ModuleKind,
}

#[derive(Clone, Debug)]
pub enum ModuleKind {
    Extern,
    Inline(Module),
}

impl Parser<'_> {
    #[inline]
    pub fn parse_module_item(&mut self) -> ParseResult<ModuleItem> {
        let module_token = self.bump_expect(&tk::kw_module)?;

        if self.bump_if_eq(tk::open_brace).is_none() {
            return Ok(ModuleItem {
                span: module_token.span,
                kind: ModuleKind::Extern,
            });
        }

        let module = self.parse_module()?;
        let close_brace = self.bump_expect(&tk::close_brace)?;

        Ok(ModuleItem {
            span: module_token.span.to(close_brace.span),
            kind: ModuleKind::Inline(module),
        })
    }
}
