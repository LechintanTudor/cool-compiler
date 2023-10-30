use crate::{FrameId, ResolveContext, ScopeError, ScopeResult, TyId};
use cool_collections::define_index_newtype;
use cool_lexer::Symbol;
use std::ops::{Index, IndexMut};

define_index_newtype!(BindingId);

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
        let frame = &mut self.frames[frame_id];

        if frame.contains(binding.symbol) {
            return Err(ScopeError::SymbolAlreadyExists);
        }

        let binding_id = self.bindings.push(binding);
        frame.bindings.push((binding.symbol, binding_id));
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
    #[inline]
    fn index_mut(&mut self, binding_id: BindingId) -> &mut Self::Output {
        &mut self.bindings[binding_id]
    }
}
