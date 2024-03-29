use crate::{
    Binding, Frame, Mutability, ResolveContext, ResolveError, ResolveErrorKind, ResolveResult,
    Scope, TyId,
};
use cool_collections::id_newtype;
use cool_lexer::Symbol;
use std::ops;

id_newtype!(FrameId);
id_newtype!(BindingId);

impl ResolveContext {
    #[inline]
    pub fn add_frame(&mut self, parent: Scope) -> FrameId {
        self.frames.push(Frame::new(parent))
    }

    #[inline]
    pub fn get_parent_frame(&self, frame_id: FrameId) -> Option<FrameId> {
        match self.frames[frame_id].parent {
            Scope::Frame(parent) => Some(parent),
            Scope::Module(_) => None,
        }
    }

    pub fn insert_local_binding(
        &mut self,
        frame_id: FrameId,
        is_mutable: bool,
        symbol: Symbol,
        ty_id: Option<TyId>,
    ) -> ResolveResult<BindingId> {
        let binding_id = self.bindings.push(Binding {
            symbol,
            mutability: Mutability::local(is_mutable),
            ty_id: ty_id.unwrap_or(self.tys.consts().infer),
        });

        if !self.frames[frame_id]
            .bindings
            .insert_if_not_exists(symbol, binding_id)
        {
            return Err(ResolveError {
                symbol,
                kind: ResolveErrorKind::SymbolAlreadyDefined,
            });
        }

        Ok(binding_id)
    }

    #[inline]
    pub fn set_binding_ty(&mut self, binding_id: BindingId, ty_id: TyId) {
        self.bindings[binding_id].ty_id = ty_id;
    }

    #[inline]
    pub fn make_binding_mutable(&mut self, binding_id: BindingId) {
        self.bindings[binding_id].mutability = Mutability::Mutable
    }
}

impl ops::Index<BindingId> for ResolveContext {
    type Output = Binding;

    #[inline]
    fn index(&self, binding_id: BindingId) -> &Self::Output {
        &self.bindings[binding_id]
    }
}
