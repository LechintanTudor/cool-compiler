use cool_collections::{define_index_newtype, Arena, VecMap};
use cool_lexer::Symbol;

define_index_newtype!(CrateId);
define_index_newtype!(LocalItemId);
define_index_newtype!(ItemId);
define_index_newtype!(TyId);

pub struct ResolveContext {
    pub crates: VecMap<CrateId, Crate>,
    pub items: VecMap<ItemId, Item>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Item {
    pub crate_id: CrateId,
    pub local_id: LocalItemId,
    pub kind: ItemKind,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ItemKind {
    Ty(TyId),
}

impl ResolveContext {
    pub fn resolve_path(&self, crate_id: CrateId, path: &[Symbol]) {
        todo!()
    }
}

pub struct Crate {
    pub name: Symbol,
    pub paths: Arena<'static, LocalItemId, [Symbol]>,
    pub items: VecMap<LocalItemId, LocalItem>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum LocalItem {
    Crate(CrateId),
    Item(ItemId),
}
