use crate::{FloatTy, IntTy, ResolveContext, TyId, TyKind};

#[derive(Clone, Default, Debug)]
pub struct TyDef {
    pub size: u64,
    pub align: u64,
    pub kind: TyDefKind,
}

#[derive(Clone, Default, Debug)]
pub enum TyDefKind {
    #[default]
    Basic,
}

impl ResolveContext {
    pub fn define_ty(&mut self, ty_id: TyId) -> Option<&TyDef> {
        if self.ty_defs[ty_id].is_some() {
            return self.ty_defs[ty_id].as_ref();
        }

        let ty_def = match &self.tys[ty_id] {
            TyKind::Unit => {
                TyDef {
                    size: 0,
                    align: 1,
                    kind: TyDefKind::Basic,
                }
            }
            TyKind::Bool => {
                TyDef {
                    size: self.ty_config.i8_size,
                    align: self.ty_config.i8_align,
                    kind: TyDefKind::Basic,
                }
            }
            TyKind::Char => {
                TyDef {
                    size: self.ty_config.i32_size,
                    align: self.ty_config.i32_align,
                    kind: TyDefKind::Basic,
                }
            }
            TyKind::Int(int_ty) => {
                let ty_config = &self.ty_config;

                let (size, align) = match int_ty {
                    IntTy::I8 | IntTy::U8 => (ty_config.i8_size, ty_config.i8_align),
                    IntTy::I16 | IntTy::U16 => (ty_config.i16_size, ty_config.i16_align),
                    IntTy::I32 | IntTy::U32 => (ty_config.i32_size, ty_config.i32_align),
                    IntTy::I64 | IntTy::U64 => (ty_config.i64_size, ty_config.i64_align),
                    IntTy::I128 | IntTy::U128 => (ty_config.i128_size, ty_config.i128_align),
                    IntTy::Usize | IntTy::Isize => (ty_config.ptr_size, ty_config.ptr_align),
                };

                TyDef {
                    size,
                    align,
                    kind: TyDefKind::Basic,
                }
            }
            TyKind::Float(float_ty) => {
                let ty_config = &self.ty_config;

                let (size, align) = match float_ty {
                    FloatTy::F32 => (ty_config.f32_size, ty_config.f32_align),
                    FloatTy::F64 => (ty_config.f64_size, ty_config.f64_align),
                };

                TyDef {
                    size,
                    align,
                    kind: TyDefKind::Basic,
                }
            }
            TyKind::Ptr(_) | TyKind::ManyPtr(_) | TyKind::Fn(_) => {
                TyDef {
                    size: self.ty_config.ptr_size,
                    align: self.ty_config.ptr_align,
                    kind: TyDefKind::Basic,
                }
            }
            ty_kind => todo!("TyDef: {:#?}", ty_kind),
        };

        self.ty_defs[ty_id] = Some(ty_def);
        self.ty_defs[ty_id].as_ref()
    }
}
