use crate::resolve::fn_ty::resolve_fn_abi;
use crate::{AstError, AstGenerator, AstResult};
use cool_parser::{ItemKind, Ty};
use cool_resolve::{FrameId, ItemPathBuf, ResolveError, ResolveErrorKind, Scope, TyId};
mod fn_ty;
use cool_span::Section;
use smallvec::SmallVec;

type TySmallVec = SmallVec<[TyId; 7]>;

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

                let param_ty_ids = fn_ty
                    .param_list
                    .params
                    .iter()
                    .map(|ty| self.resolve_ty_inner(scope, ty))
                    .collect::<Result<TySmallVec, _>>()?;

                let ret_ty_id = fn_ty
                    .ret_ty
                    .as_ref()
                    .map(|ty| self.resolve_ty_inner(scope, ty))
                    .transpose()?
                    .unwrap_or(self.tys().unit);

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
                self.resolve.mk_many_ptr(pointee, many_ptr_ty.is_mutable)
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
                self.resolve.mk_ptr(pointee, ptr_ty.is_mutable)
            }
            Ty::Slice(slice_ty) => {
                let elem = self.resolve_ty_inner(scope, &slice_ty.elem)?;
                self.resolve.mk_slice(elem, slice_ty.is_mutable)
            }
            Ty::Tuple(tuple_ty) => {
                let elem_tys = tuple_ty
                    .elems
                    .iter()
                    .map(|ty| self.resolve_ty_inner(scope, ty))
                    .collect::<Result<TySmallVec, _>>()?;

                self.resolve.mk_tuple(elem_tys)
            }
            Ty::Variant(variant_ty) => {
                let variants = variant_ty
                    .variants
                    .iter()
                    .map(|ty| self.resolve_ty_inner(scope, ty))
                    .collect::<Result<TySmallVec, _>>()?;

                self.resolve.mk_variant(variants)
            }
        };

        Ok(ty_id)
    }
}
