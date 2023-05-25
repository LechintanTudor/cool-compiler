use crate::SourceMap;
use cool_parser::{AliasItem, ConstItem, ExternFnItem, StructItem, Ty};
use cool_resolve::{ItemId, ModuleId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct Item<I> {
    pub span: Span,
    pub module_id: ModuleId,
    pub item_id: ItemId,
    pub ty: Option<Ty>,
    pub item: I,
}

impl<I> Section for Item<I> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

pub type Alias = Item<AliasItem>;
pub type Struct = Item<StructItem>;
pub type ExternFn = Item<ExternFnItem>;
pub type Const = Item<ConstItem>;

#[derive(Clone, Default, Debug)]
pub struct Package {
    pub source_map: SourceMap,
    pub aliases: Vec<Alias>,
    pub structs: Vec<Struct>,
    pub extern_fns: Vec<ExternFn>,
    pub consts: Vec<Const>,
}
