use cool_resolve::resolve::BindingId;
use cool_resolve::ty::TyId;
use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct FnParamAst {
    pub binding_id: BindingId,
    pub ty_id: TyId,
}

#[derive(Clone, Debug)]
pub struct FnPrototypeAst {
    pub params: SmallVec<[FnParamAst; 3]>,
    pub ret_ty_id: Option<TyId>,
}
