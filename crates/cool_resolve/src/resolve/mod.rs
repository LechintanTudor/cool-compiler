mod resolve_error;
mod resolve_types;
mod symbol_adder;
mod symbol_resolver;

pub use self::resolve_error::*;
pub use self::resolve_types::*;
pub use self::symbol_adder::*;
pub use self::symbol_resolver::*;
use cool_arena::SliceArena;
use cool_collections::IdIndexedVec;
use cool_lexer::symbols::Symbol;
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct ResolveTable {
    items: SliceArena<ItemId, Symbol>,
    modules: FxHashMap<ModuleId, Module>,
    frames: IdIndexedVec<FrameId, Frame>,
    bindings: IdIndexedVec<BindingId, Binding>,
    global_bindings: FxHashMap<ItemId, BindingId>,
}

impl Default for ResolveTable {
    fn default() -> Self {
        Self {
            items: Default::default(),
            modules: Default::default(),
            frames: IdIndexedVec::new(Frame {
                parent_id: ScopeId::Module(ModuleId::for_builtins()),
                bindings: Default::default(),
            }),
            bindings: Default::default(),
            global_bindings: Default::default(),
        }
    }
}
