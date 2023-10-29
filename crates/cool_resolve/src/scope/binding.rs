use crate::{FrameId, ResolveContext, ScopeResult, TyId};
use cool_arena::define_arena_index;
use cool_lexer::Symbol;
use std::ops::{Index, IndexMut};

define_arena_index!(BindingId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Mutability {
    Const,
    Immutable,
    Mutable,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Binding {
    pub symbol: Symbol,
    pub mutability: Mutability,
    pub ty_id: TyId,
}

impl ResolveContext<'_> {
    pub fn add_binding(&mut self, frame_id: FrameId, binding: Binding) -> ScopeResult<BindingId> {
        let binding_id = self.bindings.push(binding);
        self.frames[frame_id].insert(binding.symbol, binding_id)?;
        Ok(binding_id)
    }
}

impl Index<BindingId> for ResolveContext<'_> {
    type Output = Binding;

    #[inline]
    fn index(&self, binding_id: BindingId) -> &Self::Output {
        &self.bindings[binding_id]
    }
}

impl IndexMut<BindingId> for ResolveContext<'_> {
    fn index_mut(&mut self, binding_id: BindingId) -> &mut Self::Output {
        &mut self.bindings[binding_id]
    }
}
