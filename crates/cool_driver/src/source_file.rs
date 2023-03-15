use crate::ModulePaths;
use cool_lexer::lexer::LexedSourceFile;
use cool_parser::ModuleContent;
use cool_resolve::item::ItemId;
use cool_span::Span;

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub paths: ModulePaths,
    pub lexed: LexedSourceFile,
    pub module_id: ItemId,
    pub module: ModuleContent,
}

impl SourceFile {
    pub fn span_content(&self, span: Span) -> &str {
        let range = (span.start as usize)..(span.end() as usize);
        std::str::from_utf8(&self.lexed.source.as_bytes()[range]).expect("invalid span")
    }
}
