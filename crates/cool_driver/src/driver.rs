use crate::error::{DriverError, DriverResult};
use cool_lexer::lexer::{LineOffsets, Tokenizer};
use cool_lexer::symbols::Symbol;
use cool_parser::item::{Item, ModuleContent};
use cool_parser::parser::Parser;
use cool_resolve::ItemTable;
use cool_span::Span;
use smallvec::{smallvec, SmallVec};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
    pub line_offsets: LineOffsets,
    pub module: ModuleContent,
}

impl SourceFile {
    pub fn from_path(root_module: &str, path: PathBuf) -> DriverResult<Self> {
        let content = std::fs::read_to_string(&path).map_err(DriverError::SourceNotFound)?;

        let mut line_offsets = LineOffsets::default();
        let mut tokenizer = Tokenizer::new(&content, &mut line_offsets);

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

        let root_symbol = Symbol::insert(root_module);

        let mut items = ItemTable::default();
        let mut modules_to_process = Vec::<(SmallVec<[_; 4]>, _)>::new();
        modules_to_process.push((smallvec![root_symbol], &module));

        while let Some((path, module)) = modules_to_process.pop() {
            let mut builder = items.build_module(&path);

            for decl in module.decls.iter() {
                if let Item::Module(ref child_module) = decl.item {
                    let mut child_path = path.clone();
                    child_path.push(decl.ident);
                    modules_to_process.push((child_path, &child_module.content));
                } else {
                    builder.add_item(decl.ident);
                }
            }
        }

        for item in items.iter() {
            for symbol in item {
                print!("{} ", symbol);
            }

            println!();
        }

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
