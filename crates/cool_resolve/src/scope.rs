use crate::{BindingId, FrameId, ModuleId};
use cool_collections::SmallVecMap;
use cool_lexer::symbols::Symbol;

#[derive(Clone, Copy, Debug)]
pub enum ScopeId {
    Frame(FrameId),
    Module(ModuleId),
}

impl From<FrameId> for ScopeId {
    #[inline]
    fn from(frame_id: FrameId) -> Self {
        Self::Frame(frame_id)
    }
}

impl From<ModuleId> for ScopeId {
    #[inline]
    fn from(module_id: ModuleId) -> Self {
        Self::Module(module_id)
    }
}

#[derive(Clone, Debug)]
pub struct Frame {
    pub parent_id: ScopeId,
    pub bindings: SmallVecMap<Symbol, BindingId, 2>,
}

impl Frame {
    #[inline]
    pub fn new(parent_id: ScopeId) -> Self {
        Self {
            parent_id,
            bindings: Default::default(),
        }
    }
}
