use crate::{CompileError, CompileResult, LineOffsets, ModulePaths};
use cool_collections::{define_index_newtype, VecMap};
use cool_parser::{AliasItem, ExternFnExpr, FnExpr, LiteralExpr, StructItem, Ty};
use cool_resolve::{ItemId, ModuleId};
use cool_span::Span;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

define_index_newtype!(SourceId);

pub type ParsedAlias = ParsedItem<AliasItem>;
pub type ParsedExternFn = ParsedItem<ExternFnExpr>;
pub type ParsedFn = ParsedItem<FnExpr>;
pub type ParsedLiteral = ParsedItem<LiteralExpr>;
pub type ParsedStruct = ParsedItem<StructItem>;

#[derive(Default, Debug)]
pub struct ParsedCrate {
    pub files: VecMap<SourceId, SourceFile>,
    pub aliases: Vec<ParsedAlias>,
    pub structs: Vec<ParsedStruct>,
    pub literals: Vec<ParsedLiteral>,
    pub extern_fns: Vec<ParsedExternFn>,
    pub fns: Vec<ParsedFn>,
}

#[derive(Clone)]
pub struct SourceFile {
    pub paths: ModulePaths,
    pub source: String,
    pub line_offsets: LineOffsets,
}

impl SourceFile {
    pub fn from_paths(paths: ModulePaths) -> CompileResult<Self> {
        let mut file_reader = File::open(&paths.path)
            .map(BufReader::new)
            .map_err(CompileError::Io)?;

        let mut source = String::new();
        let mut line_offsets = LineOffsets::default();

        loop {
            match file_reader.read_line(&mut source) {
                Ok(0) => break,
                Ok(n) => line_offsets.add_line(n as u32),
                Err(error) => Err(CompileError::Io(error))?,
            }
        }

        Ok(Self {
            paths,
            source,
            line_offsets,
        })
    }
}

impl fmt::Debug for SourceFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceFile")
            .field("paths", &self.paths)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Debug)]
pub struct ParsedItem<I> {
    pub source_id: SourceId,
    pub span: Span,
    pub module_id: ModuleId,
    pub item_id: ItemId,
    pub ty: Option<Box<Ty>>,
    pub item: I,
}
