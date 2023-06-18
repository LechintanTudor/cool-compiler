use crate::SourceMap;
use cool_parser::{AliasItem, ConstItem, EnumItem, ExternFnItem, StructItem, Ty};
use cool_resolve::{ItemId, ModuleId};
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct DefineItem<I> {
    pub span: Span,
    pub module_id: ModuleId,
    pub item_id: ItemId,
    pub ty: Option<Ty>,
    pub item: I,
}

impl<I> Section for DefineItem<I> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

pub type Alias = DefineItem<AliasItem>;
pub type Struct = DefineItem<StructItem>;
pub type Enum = DefineItem<EnumItem>;
pub type ExternFn = DefineItem<ExternFnItem>;
pub type Const = DefineItem<ConstItem>;

#[derive(Clone, Default, Debug)]
pub struct Package {
    pub source_map: SourceMap,
    pub aliases: Vec<Alias>,
    pub enums: Vec<Enum>,
    pub structs: Vec<Struct>,
    pub extern_fns: Vec<ExternFn>,
    pub consts: Vec<Const>,
}
