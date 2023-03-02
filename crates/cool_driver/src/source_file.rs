use cool_lexer::lexer::LineOffsets;
use cool_parser::item::ModuleContent;
use cool_span::Span;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub module_path: PathBuf,
    pub child_module_dir: PathBuf,
    pub content: String,
    pub line_offsets: LineOffsets,
    pub module: ModuleContent,
}

impl SourceFile {
    pub fn span_content(&self, span: Span) -> &str {
        let range = (span.start as usize)..(span.end() as usize);
        std::str::from_utf8(&self.content.as_bytes()[range]).expect("invalid span")
    }
}
