use crate::{ResolveContext, TyId, TyKind};

impl ResolveContext<'_> {
    pub fn add_ty<K>(&mut self, kind: K) -> TyId
    where
        K: Into<TyKind>,
    {
        self.tys.insert(kind.into())
    }
}
