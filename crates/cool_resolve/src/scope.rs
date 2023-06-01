use crate::{BindingId, FrameId, ModuleId};
use cool_collections::SmallVecMap;
use cool_lexer::Symbol;
use derive_more::From;

#[derive(Clone, Copy, From, Debug)]
pub enum Scope {
    Frame(FrameId),
    Module(ModuleId),
}

#[derive(Clone, Debug)]
pub struct Frame {
    pub parent: Scope,
    pub bindings: SmallVecMap<Symbol, BindingId, 2>,
}

impl Frame {
    #[inline]
    pub fn new(parent: Scope) -> Self {
        Self {
            parent,
            bindings: Default::default(),
        }
    }
}
