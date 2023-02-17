use crate::error::{DriverError, DriverResult};
use cool_lexer::lexer::{LineOffsets, Tokenizer};
use cool_lexer::symbols;
use cool_parser::item::ModuleContent;
use cool_parser::parser::Parser;
use cool_span::Span;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
    pub line_offsets: LineOffsets,
    pub module: ModuleContent,
}

impl SourceFile {
    pub fn from_path(path: PathBuf) -> DriverResult<Self> {
        let content = std::fs::read_to_string(&path).map_err(DriverError::SourceNotFound)?;

        let mut line_offsets = LineOffsets::default();
        let mut symbol_table = symbols::write_symbol_table();
        let mut tokenizer = Tokenizer::new(&content, &mut line_offsets, &mut symbol_table);

        let token_iter = || loop {
            let token = tokenizer.next_token();

            if token.kind.is_lang_part() {
                return Some(token);
            }
        };

        let mut parser = Parser::new(std::iter::from_fn(token_iter));
        let module = parser
            .parse_module_file()
            .expect("failed to parse module file");

        Ok(Self {
            path,
            content,
            line_offsets,
            module,
        })
    }

    pub fn span_content(&self, span: Span) -> &str {
        let range = (span.start as usize)..(span.end() as usize);

        std::str::from_utf8(&self.content.as_bytes()[range]).expect("invalid span")
    }
}
