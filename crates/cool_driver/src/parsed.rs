use crate::LineOffsets;
use cool_collections::{define_index_newtype, VecMap};
use cool_parser::{FnExpr, StructItem, Ty};
use cool_resolve::{ItemId, ModuleId, ResolveContext};
use cool_span::Span;

define_index_newtype!(SourceId);

pub type ParsedStruct = ParsedItem<StructItem>;
pub type ParsedFn = ParsedItem<FnExpr>;

#[derive(Debug)]
pub struct ParsedCrate {
    pub files: VecMap<SourceId, ParsedSourceFile>,
    pub context: ResolveContext<'static>,
    pub structs: Vec<ParsedStruct>,
    pub fns: Vec<ParsedFn>,
}

#[derive(Clone, Debug)]
pub struct ParsedSourceFile {
    pub path: String,
    pub children_path: String,
    pub source: String,
    pub line_offsets: LineOffsets,
}

#[derive(Clone, Debug)]
pub struct ParsedItem<I> {
    pub source_id: SourceId,
    pub span: Span,
    pub parent_id: ModuleId,
    pub item_id: ItemId,
    pub ty: Option<Ty>,
    pub item: I,
}
