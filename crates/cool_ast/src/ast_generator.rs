use crate::resolve_ty;
use cool_parser::Ty;
use cool_resolve::{ResolveContext, ResolveResult, Scope, TyId};

pub struct AstGenerator<'a> {
    pub resolve: &'a mut ResolveContext,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(resolve: &'a mut ResolveContext) -> Self {
        Self { resolve }
    }

    #[inline]
    pub fn resolve_ty(&mut self, scope: Scope, parsed_ty: &Ty) -> ResolveResult<TyId> {
        resolve_ty(self.resolve, scope, parsed_ty)
    }
}
