use crate::resolve::fn_ty::resolve_fn_abi;
use crate::{AstError, AstGenerator, AstResult};
use cool_parser::{ItemKind, Ty};
use cool_resolve::{FrameId, ItemPathBuf, ResolveError, ResolveErrorKind, Scope, TyId};
mod fn_ty;
use cool_span::Section;
use smallvec::SmallVec;

impl AstGenerator<'_> {
    pub fn resolve_ty<S>(&mut self, scope: S, ty: &Ty) -> AstResult<TyId>
    where
        S: Into<Scope>,
    {
        self.resolve_ty_inner(scope.into(), ty)
    }

    fn resolve_ty_inner(&mut self, scope: Scope, ty: &Ty) -> AstResult<TyId> {
        let ty_id = match ty {
            Ty::Array(array_ty) => {
                let len = self
                    .gen_literal_expr(FrameId::dummy(), self.tys().usize, &array_ty.len)
                    .unwrap()
                    .as_int_value()
                    .unwrap() as u64;

                let elem = self.resolve_ty_inner(scope, &array_ty.elem)?;
                self.resolve.mk_array(len, elem)
            }
            Ty::Fn(fn_ty) => {
                let abi = resolve_fn_abi(&fn_ty.extern_decl)?;
                let mut param_ty_ids = SmallVec::<[TyId; 6]>::new();

                for param in fn_ty.param_list.params.iter() {
                    param_ty_ids.push(self.resolve_ty_inner(scope, param)?);
                }

                let ret_ty_id = match &fn_ty.ret_ty {
                    Some(ret_ty) => self.resolve_ty_inner(scope, ret_ty)?,
                    None => self.tys().unit,
                };

                self.resolve
                    .mk_fn(abi, param_ty_ids, fn_ty.param_list.is_variadic, ret_ty_id)
            }
            Ty::Item(item_ty) => {
                match item_ty.kind {
                    ItemKind::Module => self.tys().module,
                    ItemKind::Ty => self.tys().ty,
                }
            }
            Ty::ManyPtr(many_ptr_ty) => {
                let pointee = self.resolve_ty_inner(scope, &many_ptr_ty.pointee)?;
                self.resolve.mk_many_ptr(many_ptr_ty.is_mutable, pointee)
            }
            Ty::Paren(paren_ty) => self.resolve_ty_inner(scope, &paren_ty.inner)?,
            Ty::Path(path_ty) => {
                let path = path_ty
                    .idents
                    .iter()
                    .map(|ident| ident.symbol)
                    .collect::<ItemPathBuf>();

                let item_id = self
                    .resolve
                    .resolve_global(scope, &path)
                    .map_err(|error| AstError::new(path_ty.span(), error))?;

                self.resolve[item_id]
                    .as_ty_id()
                    .filter(|ty_id| !ty_id.is_infer())
                    .ok_or(AstError::new(
                        path_ty.span(),
                        ResolveError {
                            symbol: path.last(),
                            kind: ResolveErrorKind::SymbolNotTy,
                        },
                    ))?
            }
            Ty::Ptr(ptr_ty) => {
                let pointee = self.resolve_ty_inner(scope, &ptr_ty.pointee)?;
                self.resolve.mk_ptr(ptr_ty.is_mutable, pointee)
            }
            Ty::Slice(slice_ty) => {
                let elem = self.resolve_ty_inner(scope, &slice_ty.elem)?;
                self.resolve.mk_slice(slice_ty.is_mutable, elem)
            }
            Ty::Tuple(tuple_ty) => {
                let mut elem_tys = SmallVec::<[TyId; 6]>::new();

                for ty in tuple_ty.elems.iter() {
                    elem_tys.push(self.resolve_ty_inner(scope, ty)?);
                }

                self.resolve.mk_tuple(elem_tys)
            }
        };

        Ok(ty_id)
    }
}
