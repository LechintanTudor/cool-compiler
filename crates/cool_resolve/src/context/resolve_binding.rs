use crate::{
    tys, Binding, Frame, Mutability, ResolveContext, ResolveError, ResolveErrorKind, ResolveResult,
    Scope, TyId,
};
use cool_collections::id_newtype;
use cool_lexer::symbols::Symbol;
use std::ops;

id_newtype!(FrameId);
id_newtype!(BindingId);

impl ResolveContext {
    #[inline]
    pub fn add_frame(&mut self, parent: Scope) -> FrameId {
        self.frames.push(Frame::new(parent))
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
            ty_id: ty_id.unwrap_or(tys::INFERRED),
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
        let old_ty_id = &mut self.bindings[binding_id].ty_id;
        *old_ty_id = old_ty_id.resolve_non_inferred(ty_id).unwrap();
    }
}

impl ops::Index<BindingId> for ResolveContext {
    type Output = Binding;

    #[inline]
    fn index(&self, binding_id: BindingId) -> &Self::Output {
        &self.bindings[binding_id]
    }
}
