use crate::{
    resolve_fields_size_align, AnyTy, ArrayTy, EnumTy, Field, FloatTy, FnTy, IntTy, ManyPtrTy,
    PrimitiveTyData, PtrTy, ResolveTy, SliceTy, StructTy, TupleTy, VariantTy,
};
use cool_lexer::Symbol;
use derive_more::{Display, From};
use paste::paste;

macro_rules! define_value_ty {
    {
        Simple { $($SimpleTy:ident => $display:literal,)* },
        Wrapped { $($WrappedTy:ident,)* },
    } => {
        paste! {
            #[derive(Clone, PartialEq, Eq, Hash, From, Display, Debug)]
            pub enum ValueTy {
                $(
                    #[display(fmt = $display)]
                    $SimpleTy,
                )*
                $($WrappedTy([<$WrappedTy Ty>]),)*
            }

            impl ValueTy {
                $(
                    #[inline]
                    pub fn [<is_ $SimpleTy:snake:lower>](&self) -> bool {
                        matches!(self, Self::$SimpleTy)
                    }
                )*

                $(
                    #[inline]
                    pub fn [<is_ $WrappedTy:snake:lower>](&self) -> bool {
                        matches!(self, Self::$WrappedTy(_))
                    }

                    #[inline]
                    pub fn [<as_ $WrappedTy:snake:lower>](&self) -> Option<&[<$WrappedTy Ty>]> {
                        match self {
                            Self::$WrappedTy(ty) => Some(ty),
                            _ => None,
                        }
                    }
                )*
            }

            impl AnyTy {
                $(
                    #[inline]
                    pub fn [<is_ $SimpleTy:snake:lower>](&self) -> bool {
                        matches!(self, Self::Value(ValueTy::$SimpleTy))
                    }
                )*

                $(
                    #[inline]
                    pub fn [<is_ $WrappedTy:snake:lower>](&self) -> bool {
                        matches!(self, Self::Value(ValueTy::$WrappedTy(_)))
                    }

                    #[inline]
                    pub fn [<as_ $WrappedTy:snake:lower>](&self) -> Option<&[<$WrappedTy Ty>]> {
                        match self {
                            Self::Value(ValueTy::$WrappedTy(ty)) => Some(ty),
                            _ => None,
                        }
                    }
                )*
            }
        }
    };
}

define_value_ty! {
    Simple {
        Unit => "()",
        Bool => "bool",
        Char => "char",
    },
    Wrapped {
        Int,
        Float,
        Array,
        Tuple,
        Struct,
        Enum,
        Fn,
        Ptr,
        ManyPtr,
        Slice,
        Variant,
    },
}

impl ValueTy {
    #[inline]
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Int(_) | Self::Float(_))
    }

    #[inline]
    pub fn is_signed_int(&self) -> bool {
        self.as_int().is_some_and(IntTy::is_signed)
    }

    #[inline]
    pub fn is_unsigned_int(&self) -> bool {
        self.as_int().is_some_and(IntTy::is_unsigned)
    }

    #[inline]
    pub fn is_usize(&self) -> bool {
        matches!(self, Self::Int(IntTy::Usize))
    }

    #[inline]
    pub fn is_comparable(&self) -> bool {
        matches!(
            self,
            Self::Int(_) | Self::Float(_) | Self::Ptr(_) | Self::ManyPtr(_),
        )
    }

    pub fn get_aggregate_field(&self, symbol: Symbol) -> Option<Field> {
        match self {
            Self::Tuple(tuple_ty) => {
                tuple_ty
                    .fields
                    .iter()
                    .find(|field| field.symbol == symbol)
                    .cloned()
            }
            Self::Struct(struct_ty) => {
                let def = struct_ty.def.lock().unwrap();
                let fields: &[_] = &def.as_ref().unwrap().fields;
                fields.iter().find(|field| field.symbol == symbol).cloned()
            }
            Self::Slice(slice_ty) => {
                slice_ty
                    .fields
                    .iter()
                    .find(|field| field.symbol == symbol)
                    .cloned()
            }
            _ => None,
        }
    }

    pub fn to_resolve_ty(self, primitives: &PrimitiveTyData) -> ResolveTy {
        match self {
            Self::Unit => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    ty: AnyTy::Value(self),
                }
            }
            Self::Bool => {
                ResolveTy {
                    size: 1,
                    align: 1,
                    ty: AnyTy::Value(self),
                }
            }
            Self::Char => {
                ResolveTy {
                    size: 4,
                    align: primitives.i32_align,
                    ty: AnyTy::Value(self),
                }
            }
            Self::Int(int_ty) => int_ty.to_resolve_ty(primitives),
            Self::Float(float_ty) => float_ty.to_resolve_ty(primitives),
            Self::Array(array_ty) => {
                let elem_size = array_ty.elem.get_size();
                let elem_align = array_ty.elem.get_align();

                ResolveTy {
                    size: elem_size * array_ty.len,
                    align: elem_align,
                    ty: AnyTy::Value(Self::Array(array_ty)),
                }
            }
            Self::Tuple(mut tuple_ty) => {
                if tuple_ty.fields.is_empty() {
                    return ResolveTy {
                        size: 0,
                        align: 1,
                        ty: AnyTy::Value(Self::Unit),
                    };
                }

                let (size, align) = resolve_fields_size_align(&mut tuple_ty.fields);

                ResolveTy {
                    size,
                    align,
                    ty: AnyTy::Value(Self::Tuple(tuple_ty)),
                }
            }
            Self::Struct(_) => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    ty: AnyTy::Value(self),
                }
            }
            Self::Enum(enum_ty) => {
                let storage_ty = enum_ty.storage.as_int().unwrap().to_resolve_ty(primitives);

                ResolveTy {
                    ty: AnyTy::Value(Self::Enum(enum_ty)),
                    ..storage_ty
                }
            }
            Self::Fn(_) | Self::Ptr(_) | Self::ManyPtr(_) => {
                ResolveTy {
                    size: primitives.ptr_size,
                    align: primitives.ptr_align,
                    ty: AnyTy::Value(self),
                }
            }
            Self::Slice(mut slice_ty) => {
                let (size, align) = resolve_fields_size_align(&mut slice_ty.fields);

                ResolveTy {
                    size,
                    align,
                    ty: AnyTy::Value(self),
                }
            }
            Self::Variant(variant_ty) => variant_ty.to_resolve_ty(),
        }
    }
}
