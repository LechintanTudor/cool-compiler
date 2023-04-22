use crate::resolve_ty;
use cool_parser::Ty;
use cool_resolve::{ResolveContext, ResolveResult, ScopeId, TyId};

pub struct AstGenerator<'a> {
    pub resolve: &'a mut ResolveContext,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(resolve: &'a mut ResolveContext) -> Self {
        Self { resolve }
    }

    #[inline]
    pub fn resolve_ty(&mut self, scope_id: ScopeId, parsed_ty: &Ty) -> ResolveResult<TyId> {
        resolve_ty(self.resolve, scope_id, parsed_ty)
    }
}
