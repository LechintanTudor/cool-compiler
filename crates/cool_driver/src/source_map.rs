use cool_lexer::lexer::{TokenStream, Tokenizer};
use cool_parser::{ModuleContent, ParseResult, Parser};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub path: PathBuf,
    pub source: String,
}

#[derive(Clone, Default, Debug)]
pub struct SourceMap {
    line_offsets: Vec<u32>,
    files: Vec<SourceFile>,
}

impl SourceMap {
    pub fn add_file(&mut self, path: PathBuf) -> ParseResult<ModuleContent> {
        let file = File::open(&path).unwrap();
        let mut buf_reader = BufReader::new(file);

        let offset = self.line_offsets.last().copied().unwrap_or(0);
        let mut source = String::new();

        loop {
            match buf_reader.read_line(&mut source) {
                Ok(0) => break,
                Ok(n) => self.line_offsets.push(offset + n as u32),
                Err(error) => todo!("Error reading file: {error}"),
            }
        }

        let mut tokenizer = Tokenizer::new(&source, offset);
        let mut parser = Parser::new(TokenStream::new(&mut tokenizer));
        let module_content = parser.parse_module_file()?;

        self.files.push(SourceFile { path, source });
        Ok(module_content)
    }
}
