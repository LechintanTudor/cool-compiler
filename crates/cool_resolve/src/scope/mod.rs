mod binding;
mod scope_error;

pub use self::binding::*;
pub use self::scope_error::*;

use crate::{ModuleId, ResolveContext};
use cool_arena::define_arena_index;
use cool_lexer::Symbol;
use smallvec::SmallVec;
use std::ops::{Index, IndexMut};

define_arena_index!(FrameId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Scope {
    Module(ModuleId),
    Frame(FrameId),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Frame {
    pub parent: Scope,
    pub bindings: SmallVec<[(Symbol, BindingId); 2]>,
}

impl Frame {
    pub fn insert(&mut self, symbol: Symbol, binding: BindingId) -> ScopeResult<()> {
        if self.bindings.iter().any(|(s, _)| *s == symbol) {
            return Err(ScopeError::SymbolAlreadyExists);
        }

        self.bindings.push((symbol, binding));
        Ok(())
    }

    #[must_use]
    pub fn get(&self, symbol: Symbol) -> Option<BindingId> {
        self.bindings
            .iter()
            .find(|(s, _)| *s == symbol)
            .map(|(_, binding)| *binding)
    }
}

impl ResolveContext<'_> {
    pub fn add_frame<S>(&mut self, parent: S) -> FrameId
    where
        S: Into<Scope>,
    {
        self.frames.push(Frame {
            parent: parent.into(),
            bindings: Default::default(),
        })
    }
}

impl Index<FrameId> for ResolveContext<'_> {
    type Output = Frame;

    #[inline]
    fn index(&self, frame_id: FrameId) -> &Self::Output {
        &self.frames[frame_id]
    }
}

impl IndexMut<FrameId> for ResolveContext<'_> {
    #[inline]
    fn index_mut(&mut self, frame_id: FrameId) -> &mut Self::Output {
        &mut self.frames[frame_id]
    }
}
