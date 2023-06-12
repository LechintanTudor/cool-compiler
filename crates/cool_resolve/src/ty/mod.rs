mod any_ty;
mod array_ty;
mod consts;
mod field;
mod infer_ty;
mod item_ty;
mod primitive_ty;
mod ptr_ty;
mod resolve_ty;
mod struct_ty;
mod tuple_ty;
mod ty_id;
mod value_ty;

pub use self::any_ty::*;
pub use self::array_ty::*;
pub use self::consts::*;
pub use self::field::*;
pub use self::infer_ty::*;
pub use self::item_ty::*;
pub use self::primitive_ty::*;
pub use self::ptr_ty::*;
pub use self::resolve_ty::*;
pub use self::struct_ty::*;
pub use self::tuple_ty::*;
pub use self::ty_id::*;
pub use self::value_ty::*;
use cool_arena::Arena;
use cool_collections::id_newtype;

id_newtype!(InternalTyId);

pub(crate) type TyArena = Arena<'static, InternalTyId, ResolveTy>;

#[derive(Debug)]
pub struct TyContext {
    primitives: PrimitiveTyData,
    tys: TyArena,
    consts: TyConsts,
}

impl TyContext {
    pub fn new(primitives: PrimitiveTyData) -> Self {
        let mut tys = TyArena::new_leak();
        let consts = TyConsts::new(&mut tys, &primitives);

        Self {
            primitives,
            tys,
            consts,
        }
    }

    pub fn get_or_insert(&mut self, ty: AnyTy) -> TyId {
        let ty: ResolveTy = match ty {
            AnyTy::Infer(ty) => ty.into(),
            AnyTy::Item(ty) => ty.into(),
            AnyTy::Value(ty) => ty.to_resolve_ty(&self.primitives),
        };

        let internal_ty_id = self.tys.get_or_insert(ty);
        TyId::new(self.tys.get(internal_ty_id).unwrap())
    }

    #[inline]
    pub fn consts(&self) -> &TyConsts {
        &self.consts
    }
}
