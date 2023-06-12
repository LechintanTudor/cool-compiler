use crate::resolve::fn_ty::resolve_fn_abi;
use crate::AstGenerator;
use cool_parser::Ty;
use cool_resolve::{
    FrameId, ItemPathBuf, ResolveError, ResolveErrorKind, ResolveResult, Scope, TyId,
};
mod fn_ty;
use smallvec::SmallVec;

impl AstGenerator<'_> {
    pub fn resolve_ty(&mut self, scope: Scope, ty: &Ty) -> ResolveResult<TyId> {
        let ty_id = match ty {
            Ty::Fn(fn_ty) => {
                let abi = resolve_fn_abi(&fn_ty.extern_decl)?;
                let mut param_ty_ids = SmallVec::<[TyId; 6]>::new();

                for param in fn_ty.param_list.params.iter() {
                    param_ty_ids.push(self.resolve_ty(scope, param)?);
                }

                let ret_ty_id = match &fn_ty.ret_ty {
                    Some(ret_ty) => self.resolve_ty(scope, ret_ty)?,
                    None => self.tys().unit,
                };

                self.resolve
                    .mk_fn(abi, param_ty_ids, fn_ty.param_list.is_variadic, ret_ty_id)
            }
            Ty::Path(path) => {
                let path = path
                    .idents
                    .iter()
                    .map(|ident| ident.symbol)
                    .collect::<ItemPathBuf>();

                let item_id = self.resolve.resolve_global(scope, &path)?;

                self.resolve[item_id]
                    .as_ty_id()
                    .filter(|ty_id| !ty_id.is_infer())
                    .ok_or(ResolveError {
                        symbol: path.last(),
                        kind: ResolveErrorKind::SymbolNotTy,
                    })?
            }
            Ty::Array(array_ty) => {
                let len = self
                    .gen_literal_expr(FrameId::dummy(), self.tys().usize, &array_ty.len)
                    .unwrap()
                    .as_int_value()
                    .unwrap() as u64;

                let elem = self.resolve_ty(scope, &array_ty.elem)?;
                self.resolve.mk_array(len, elem)
            }
            Ty::Ptr(ptr_ty) => {
                let pointee = self.resolve_ty(scope, &ptr_ty.pointee)?;
                self.resolve.mk_ptr(ptr_ty.is_mutable, pointee)
            }
            Ty::ManyPtr(many_ptr_ty) => {
                let pointee = self.resolve_ty(scope, &many_ptr_ty.pointee)?;
                self.resolve.mk_many_ptr(many_ptr_ty.is_mutable, pointee)
            }
            Ty::Slice(slice_ty) => {
                let elem = self.resolve_ty(scope, &slice_ty.elem)?;
                self.resolve.mk_slice(slice_ty.is_mutable, elem)
            }
            Ty::Tuple(tuple_ty) => {
                let mut elem_tys = SmallVec::<[TyId; 6]>::new();

                for ty in tuple_ty.elems.iter() {
                    elem_tys.push(self.resolve_ty(scope, ty)?);
                }

                self.resolve.mk_tuple(elem_tys)
            }
            _ => todo!(),
        };

        Ok(ty_id)
    }
}
