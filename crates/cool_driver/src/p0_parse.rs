use cool_lexer::TokenStream;
use cool_parser::{Parser, SourceFile};
use cool_resolve::{ResolveContext, TyConfig};
use std::fs;

#[derive(Debug)]
pub struct ParsedProgram {
    pub files: Vec<ParsedSourceFile>,
    pub resolve: ResolveContext<'static>,
}

#[derive(Clone, Debug)]
pub struct ParsedSourceFile {
    pub path: String,
    pub file: SourceFile,
}

pub fn p0_parse(_crate_name: &str, path: &str) -> ParsedProgram {
    let source = fs::read_to_string(path).unwrap();
    let token_stream = TokenStream::new(&source);
    let mut parser = Parser::new(token_stream);
    let source_file = parser.parse_source_file().unwrap();

    ParsedProgram {
        files: vec![ParsedSourceFile {
            path: path.to_string(),
            file: source_file,
        }],
        resolve: ResolveContext::new_leak(TyConfig { ptr_size: 8 }),
    }
}
