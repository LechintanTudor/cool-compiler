use cool_parser::Ty;
use cool_resolve::expr_ty::ExprTyTable;
use cool_resolve::resolve::{ResolveError, ResolveResult, ResolveTable, ScopeId};
use cool_resolve::ty::{TyId, TyTable};
use cool_resolve::ItemPathBuf;

pub struct AstGenerator<'a> {
    pub resolve: &'a mut ResolveTable,
    pub tys: &'a mut TyTable,
    pub expr_tys: ExprTyTable,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(resolve: &'a mut ResolveTable, tys: &'a mut TyTable) -> Self {
        Self {
            resolve,
            tys,
            expr_tys: Default::default(),
        }
    }

    pub fn resolve_ty(&mut self, scope_id: ScopeId, parsed_ty: &Ty) -> ResolveResult<TyId> {
        match parsed_ty {
            Ty::Path(path) => {
                let path = path
                    .idents
                    .iter()
                    .map(|ident| ident.symbol)
                    .collect::<ItemPathBuf>();

                let item_id = self.resolve.resolve_global(scope_id, &path)?;
                self.tys
                    .get_id_by_item_id(item_id)
                    .ok_or(ResolveError::not_ty(path.last()))
            }
            Ty::Tuple(tuple_ty) => {
                let mut elems = Vec::<TyId>::new();

                for ty in tuple_ty.elems.iter() {
                    elems.push(self.resolve_ty(scope_id, ty)?);
                }

                Ok(self.tys.mk_tuple(elems))
            }
            _ => todo!(),
        }
    }
}
