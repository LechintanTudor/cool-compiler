use cool_lexer::symbols::sym;
use cool_parser::Ty;
use cool_resolve::{tys, ItemPathBuf, ResolveContext, ResolveError, ResolveResult, ScopeId, TyId};
use smallvec::SmallVec;

pub struct AstGenerator<'a> {
    pub resolve: &'a mut ResolveContext,
}

impl<'a> AstGenerator<'a> {
    #[inline]
    pub fn new(resolve: &'a mut ResolveContext) -> Self {
        Self { resolve }
    }

    pub fn resolve_parsed_ty(&mut self, scope_id: ScopeId, parsed_ty: &Ty) -> ResolveResult<TyId> {
        match parsed_ty {
            Ty::Fn(fn_ty) => {
                let mut param_ty_ids = SmallVec::<[TyId; 6]>::new();

                let abi = fn_ty
                    .extern_decl
                    .as_ref()
                    .map(|decl| decl.abi.unwrap_or(sym::ABI_C))
                    .unwrap_or(sym::ABI_COOL);

                for param in fn_ty.param_list.params.iter() {
                    param_ty_ids.push(self.resolve_parsed_ty(scope_id, param)?);
                }

                let ret_ty_id = match &fn_ty.ret_ty {
                    Some(ret_ty) => self.resolve_parsed_ty(scope_id, ret_ty)?,
                    None => tys::UNIT,
                };

                Ok(self.resolve.mk_fn(abi, param_ty_ids, ret_ty_id))
            }
            Ty::Module(_) => Ok(tys::MODULE),
            Ty::Path(path) => {
                let path = path
                    .idents
                    .iter()
                    .map(|ident| ident.symbol)
                    .collect::<ItemPathBuf>();

                let item_id = self.resolve.resolve_global(scope_id, &path)?;

                self.resolve[item_id]
                    .as_ty_id()
                    .ok_or(ResolveError::not_ty(path.last()))
            }
            Ty::Tuple(tuple_ty) => {
                let mut elem_tys = SmallVec::<[TyId; 6]>::new();

                for ty in tuple_ty.elems.iter() {
                    elem_tys.push(self.resolve_parsed_ty(scope_id, ty)?);
                }

                Ok(self.resolve.mk_tuple(elem_tys))
            }
            _ => todo!(),
        }
    }
}
