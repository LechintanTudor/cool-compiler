mod binding;

pub use self::binding::*;

use crate::{ModuleId, ResolveContext};
use cool_collections::define_index_newtype;
use cool_lexer::Symbol;
use derive_more::From;
use smallvec::SmallVec;
use std::ops::{Index, IndexMut};

define_index_newtype!(FrameId);

#[derive(Clone, Copy, PartialEq, Eq, Hash, From, Debug)]
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
    #[must_use]
    pub fn get(&self, symbol: Symbol) -> Option<BindingId> {
        self.bindings
            .iter()
            .find(|(s, _)| *s == symbol)
            .map(|(_, binding)| *binding)
    }

    #[inline]
    #[must_use]
    pub fn contains(&self, symbol: Symbol) -> bool {
        self.bindings.iter().any(|(s, _)| *s == symbol)
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

    #[must_use]
    pub fn get_toplevel_module<S>(&self, scope: S) -> ModuleId
    where
        S: Into<Scope>,
    {
        let mut scope = scope.into();

        loop {
            match scope {
                Scope::Frame(frame_id) => scope = self.frames[frame_id].parent,
                Scope::Module(module_id) => break module_id,
            }
        }
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
