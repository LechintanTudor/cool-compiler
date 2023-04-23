use cool_parser::{AliasItem, StructItem, ConstItem, Ty};
use cool_resolve::{ItemId, ModuleId};

pub struct Alias {
    pub module_id: ModuleId,
    pub item_id: ItemId,
    pub ty: Option<Ty>,
    pub item: AliasItem,
}

pub struct Struct {
    pub module_id: ModuleId,
    pub item_id: ItemId,
    pub ty: Option<Ty>,
    pub item: StructItem,
}

pub struct Const {
    pub module_id: ModuleId,
    pub item_id: ItemId,
    pub ty: Option<Ty>,
    pub item: ConstItem,
}

pub struct Package {
    pub aliases: Vec<Alias>,
    pub structs: Vec<Struct>,
    pub consts: Vec<Const>,
}
