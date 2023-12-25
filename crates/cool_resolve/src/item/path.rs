use crate::{ItemId, ModuleId, ResolveContext, ResolveResult};
use cool_lexer::Symbol;

impl ResolveContext {
    pub fn resolve_path(&self, _module_id: ModuleId, _path: &[Symbol]) -> ResolveResult<ItemId> {
        todo!()
    }
}

trait SymbolPath {
    #[must_use]
    fn pop_front(&self) -> &Self;

    #[must_use]
    fn pop_back(&self) -> &Self;
}

impl SymbolPath for [Symbol] {
    fn pop_front(&self) -> &Self {
        match self {
            [] => &[],
            [_, tail @ ..] => tail,
        }
    }

    fn pop_back(&self) -> &Self {
        match self {
            [] => &[],
            [head @ .., _] => head,
        }
    }
}
