use cool_parser::{AliasItem, ConstItem, ExternFnItem, StructItem, Ty};
use cool_resolve::{ItemId, ModuleId};

pub struct Item<I> {
    pub module_id: ModuleId,
    pub item_id: ItemId,
    pub ty: Option<Ty>,
    pub item: I,
}

pub type Alias = Item<AliasItem>;
pub type Struct = Item<StructItem>;
pub type ExternFn = Item<ExternFnItem>;
pub type Const = Item<ConstItem>;

pub struct Package {
    pub aliases: Vec<Alias>,
    pub structs: Vec<Struct>,
    pub extern_fns: Vec<ExternFn>,
    pub consts: Vec<Const>,
}
