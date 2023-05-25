use cool_lexer::lexer::{TokenStream, Tokenizer};
use cool_parser::{ModuleContent, ParseResult, Parser};
use cool_span::{SourcePosition, Span};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{fmt, str};

#[derive(Clone)]
pub struct SourceFile {
    pub path: PathBuf,
    pub span: Span,
    pub line_offsets: Vec<u32>,
    pub source: String,
}

impl SourceFile {
    pub fn get_source_at_span(&self, span: Span) -> &str {
        let start = (span.start - self.span.start) as usize;
        let end = (span.end() - self.span.start) as usize;
        let bytes = &self.source.as_bytes()[start..end];
        str::from_utf8(bytes).unwrap()
    }

    pub fn offset_to_line(&self, offset: u32) -> u32 {
        let relative_offset = offset - self.span.start;

        self.line_offsets
            .partition_point(|&line_offset| line_offset <= relative_offset) as u32
    }

    pub fn offset_to_position(&self, offset: u32) -> SourcePosition {
        let line = self.offset_to_line(offset);
        let relative_offset = offset - self.span.start;

        let column = self
            .line_offsets
            .get((line - 1) as usize)
            .map(|line_offset| relative_offset - line_offset + 1)
            .unwrap_or(1);

        SourcePosition { line, column }
    }
}

impl fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceFile")
            .field("path", &self.path)
            .field("span", &self.span)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Default, Debug)]
pub struct SourceMap {
    files: Vec<SourceFile>,
}

impl SourceMap {
    pub fn add_file(&mut self, path: PathBuf) -> ParseResult<ModuleContent> {
        let file = File::open(&path).unwrap();
        let mut buf_reader = BufReader::new(file);

        let start_offset = self.files.last().map(|file| file.span.end()).unwrap_or(0);

        let mut line_offsets = Vec::<u32>::new();
        let mut line_offset = 0;
        let mut source = String::new();

        loop {
            match buf_reader.read_line(&mut source) {
                Ok(0) => break,
                Ok(n) => {
                    line_offsets.push(line_offset);
                    line_offset += n as u32;
                }
                Err(error) => todo!("Error reading file: {error}"),
            }
        }

        let end_offset = start_offset + source.len() as u32;
        let span = Span::new(start_offset, end_offset);

        let mut tokenizer = Tokenizer::new(&source, start_offset);
        let mut parser = Parser::new(TokenStream::new(&mut tokenizer));
        let module_content = parser.parse_module_file();

        self.files.push(SourceFile {
            path,
            span,
            line_offsets,
            source,
        });

        module_content
    }

    pub fn get_file_from_offset(&self, offset: u32) -> &SourceFile {
        self.files
            .iter()
            .rev()
            .find(|file| !file.span.is_empty() && file.span.start <= offset)
            .unwrap_or(self.files.first().unwrap())
    }

    pub fn get_file_and_position_from_offset(&self, offset: u32) -> (&SourceFile, SourcePosition) {
        let source_file = self.get_file_from_offset(offset);
        let position = source_file.offset_to_position(offset);
        (source_file, position)
    }

    pub fn get_source_at_span(&self, span: Span) -> &str {
        self.get_file_from_offset(span.start)
            .get_source_at_span(span)
    }
}
