use crate::{FloatTy, IntTy, ResolveContext, TyId, TyKind};
use cool_collections::SmallVec;
use cool_lexer::Symbol;
use derive_more::From;
use std::cmp::Reverse;

#[derive(Clone, Debug)]
pub struct TyDef {
    pub size: u64,
    pub align: u64,
    pub kind: TyDefKind,
}

#[derive(Clone, From, Debug)]
pub enum TyDefKind {
    Basic,
    Struct(StructTyDef),
    Variant(VariantTyDef),
}

#[derive(Clone, Debug)]
pub struct StructTyDef {
    pub fields: Vec<Field>,
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub offset: u64,
    pub name: Symbol,
    pub ty_id: TyId,
}

#[derive(Clone, Debug)]
pub struct VariantTyDef {
    pub variant_tys: SmallVec<TyId, 4>,
    pub index_offset: u64,
    pub index_ty: TyId,
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
            TyKind::Array(array_ty) => {
                let array_len = array_ty.len;
                let elem_ty_def = self.define_ty(array_ty.elem_ty)?;

                let size = elem_ty_def
                    .size
                    .checked_mul(array_len)
                    .expect("Array size overflow");

                TyDef {
                    size,
                    align: elem_ty_def.align,
                    kind: TyDefKind::Basic,
                }
            }
            TyKind::Tuple(tuple_ty) => {
                let fields = tuple_ty
                    .elem_tys
                    .iter()
                    .enumerate()
                    .map(|(i, &ty_id)| (Symbol::insert_fmt(&i), ty_id))
                    .collect::<SmallVec<_, 8>>();

                self.get_struct_ty_def(&fields)?
            }
            TyKind::Struct(_) => return None,
            ty_kind => todo!("TyDef: {:#?}", ty_kind),
        };

        self.ty_defs[ty_id] = Some(ty_def);
        self.ty_defs[ty_id].as_ref()
    }

    pub fn define_struct_ty(&mut self, ty_id: TyId, fields: &[(Symbol, TyId)]) -> Option<&TyDef> {
        let ty_def = self.get_struct_ty_def(fields)?;
        self.ty_defs[ty_id] = Some(ty_def);
        self.ty_defs[ty_id].as_ref()
    }

    pub fn iter_tys_to_be_defined(&self) -> impl Iterator<Item = TyId> + '_ {
        self.tys
            .iter_indexes()
            .filter(|&ty| ty.is_definable() && self.ty_defs[ty].is_none())
    }

    fn get_struct_ty_def(&mut self, fields: &[(Symbol, TyId)]) -> Option<TyDef> {
        #[derive(Clone, Copy)]
        struct DefinedField {
            name: Symbol,
            ty_id: TyId,
            size: u64,
            align: u64,
        }

        let defined_fields = {
            let mut defined_fields = Vec::<(usize, DefinedField)>::new();

            for (i, &(name, ty_id)) in fields.iter().enumerate() {
                let field_def = self.define_ty(ty_id)?;

                defined_fields.push((
                    i,
                    DefinedField {
                        name,
                        ty_id,
                        size: field_def.size,
                        align: field_def.align,
                    },
                ));
            }

            defined_fields.sort_by_key(|(_, field)| Reverse((field.align, field.size)));
            defined_fields
        };

        let (size, align, fields) = {
            let mut fields = Vec::<(usize, Field)>::new();
            let mut offset = 0;
            let mut align = 1;

            for (i, field) in defined_fields {
                fields.push((
                    i,
                    Field {
                        offset,
                        name: field.name,
                        ty_id: field.ty_id,
                    },
                ));

                offset += field.size + padding_for_align(offset, field.align);
                align = align.max(field.align);
            }

            let size = offset + padding_for_align(offset, align);
            fields.sort_by_key(|(i, _)| *i);

            let fields = fields
                .iter()
                .map(|(_, field)| {
                    Field {
                        offset: field.offset,
                        name: field.name,
                        ty_id: field.ty_id,
                    }
                })
                .collect::<Vec<_>>();

            (size, align, fields)
        };

        Some(TyDef {
            size,
            align,
            kind: StructTyDef { fields }.into(),
        })
    }
}

fn padding_for_align(offset: u64, align: u64) -> u64 {
    let misalign = offset % align;

    if misalign > 0 {
        align - misalign
    } else {
        0
    }
}
