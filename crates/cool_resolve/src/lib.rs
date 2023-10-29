mod id_vec;
mod item;
mod scope;
mod ty;

pub use self::item::*;
pub use self::scope::*;
pub use self::ty::*;

pub(crate) use self::id_vec::*;

use ahash::AHashMap;
use cool_arena::Arena;
use cool_lexer::Symbol;

#[derive(Debug)]
pub struct ResolveContext<'a> {
    paths: Arena<'a, ItemId, [Symbol]>,
    items: AHashMap<ItemId, ItemKind>,
    modules: IdVec<ModuleId, ModuleItem>,
    ty_config: TyConfig,
    tys: Arena<'a, TyId, TyKind>,
    ty_defs: AHashMap<TyId, TyDef>,
    frames: IdVec<FrameId, Frame>,
    bindings: IdVec<BindingId, Binding>,
}

impl<'a> ResolveContext<'a> {
    pub fn new_leak(ty_config: TyConfig) -> Self {
        let mut ctx = Self {
            paths: Arena::new_leak(),
            items: Default::default(),
            modules: Default::default(),
            ty_config,
            tys: Arena::new_leak(),
            ty_defs: Default::default(),
            frames: Default::default(),
            bindings: Default::default(),
        };

        ctx.init_builtins();
        ctx
    }
}
