mod resolve_error;
mod resolve_global;
mod resolve_local;
mod resolve_types;

pub use self::resolve_error::*;
pub use self::resolve_global::*;
pub use self::resolve_local::*;
pub use self::resolve_types::*;
use crate::consts::itm;
use cool_arena::SliceArena;
use cool_collections::IdIndexedVec;
use cool_lexer::symbols::{sym, Symbol};
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct ResolveTable {
    items: SliceArena<ItemId, Symbol>,
    modules: FxHashMap<ModuleId, Module>,
    frames: IdIndexedVec<FrameId, Frame>,
    bindings: IdIndexedVec<BindingId, Binding>,
}

impl ResolveTable {
    pub fn with_builtins() -> Self {
        let mut resolve = Self::default();
        assert_eq!(
            resolve.insert_root_module(sym::EMPTY).unwrap(),
            ModuleId::for_builtins()
        );
        itm::insert_builtins(&mut resolve);
        resolve
    }

    pub fn insert_builtin_item(&mut self, item_id: ItemId, symbol: Symbol) {
        assert_eq!(
            self.insert_item(ModuleId::for_builtins(), true, symbol)
                .unwrap(),
            item_id,
        );
    }
}

impl Default for ResolveTable {
    fn default() -> Self {
        Self {
            items: Default::default(),
            modules: Default::default(),
            frames: IdIndexedVec::new(Frame {
                parent_id: ScopeId::Module(ModuleId::dummy()),
                bindings: Default::default(),
            }),
            bindings: Default::default(),
        }
    }
}
