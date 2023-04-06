use cool_parser::Ty;
use cool_resolve::expr_ty::ExprTyTable;
use cool_resolve::resolve::{ResolveError, ResolveResult, ResolveTable, ScopeId};
use cool_resolve::ty::{tys, TyId, TyTable};
use cool_resolve::ItemPathBuf;
use smallvec::SmallVec;

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

    pub fn resolve_parsed_ty(&mut self, scope_id: ScopeId, parsed_ty: &Ty) -> ResolveResult<TyId> {
        match parsed_ty {
            Ty::Fn(fn_ty) => {
                let mut param_ty_ids = SmallVec::<[TyId; 6]>::new();

                for param in fn_ty.param_list.params.iter() {
                    param_ty_ids.push(self.resolve_parsed_ty(scope_id, param)?);
                }

                let ret_ty_id = match &fn_ty.ret_ty {
                    Some(ret_ty) => self.resolve_parsed_ty(scope_id, ret_ty)?,
                    None => tys::UNIT,
                };

                Ok(self.tys.mk_fn(param_ty_ids, ret_ty_id))
            }
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
                let mut elem_tys = SmallVec::<[TyId; 6]>::new();

                for ty in tuple_ty.elems.iter() {
                    elem_tys.push(self.resolve_parsed_ty(scope_id, ty)?);
                }

                Ok(self.tys.mk_tuple(elem_tys))
            }
            _ => todo!(),
        }
    }
}
