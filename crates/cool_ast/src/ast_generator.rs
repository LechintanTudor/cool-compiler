use cool_parser::Ty;
use cool_resolve::resolve::{ResolveTable, ScopeId};
use cool_resolve::ty::{TyId, TyTable};
use cool_resolve::ItemPathBuf;

pub struct AstGenerator<'a> {
    pub resolve: &'a mut ResolveTable,
    pub tys: &'a mut TyTable,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(resolve: &'a mut ResolveTable, tys: &'a mut TyTable) -> Self {
        Self { resolve, tys }
    }

    pub fn resolve_ty(&mut self, scope_id: ScopeId, parsed_ty: &Ty) -> Option<TyId> {
        match parsed_ty {
            Ty::Path(path) => {
                let path = path
                    .idents
                    .iter()
                    .map(|ident| ident.symbol)
                    .collect::<ItemPathBuf>();

                let item_id = self.resolve.resolve_path_as_item(scope_id, &path)?;
                self.tys.get_id_by_item_id(item_id)
            }
            Ty::Tuple(tuple_ty) => {
                let mut elems = Vec::<TyId>::new();

                for ty in tuple_ty.elems.iter() {
                    elems.push(self.resolve_ty(scope_id, ty)?);
                }

                Some(self.tys.mk_tuple(elems))
            }
        }
    }
}
