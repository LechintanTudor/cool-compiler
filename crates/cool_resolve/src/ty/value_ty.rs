use crate::{
    resolve_fields_size_align, AnyTy, ArrayTy, Field, FloatTy, FnTy, IntTy, ManyPtrTy,
    PrimitiveTyData, PtrTy, ResolveTy, SliceTy, StructTy, TupleTy,
};
use cool_lexer::Symbol;
use derive_more::From;
use paste::paste;
use std::fmt;

macro_rules! define_value_ty {
    { Simple { $($SimpleTy:ident,)* }, Wrapped { $($WrappedTy:ident,)* }, } => {
        paste! {
            #[derive(Clone, PartialEq, Eq, Hash, From, Debug)]
            pub enum ValueTy {
                $($SimpleTy,)*
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
        Unit,
        Bool,
        Char,
    },
    Wrapped {
        Int,
        Float,
        Array,
        Tuple,
        Struct,
        Fn,
        Ptr,
        ManyPtr,
        Slice,
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
            ValueTy::Unit => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    ty: AnyTy::Value(self),
                }
            }
            ValueTy::Bool => {
                ResolveTy {
                    size: 1,
                    align: 1,
                    ty: AnyTy::Value(self),
                }
            }
            ValueTy::Char => {
                ResolveTy {
                    size: 4,
                    align: primitives.i32_align,
                    ty: AnyTy::Value(self),
                }
            }
            ValueTy::Int(int_ty) => int_ty.to_resolve_ty(primitives),
            ValueTy::Float(float_ty) => float_ty.to_resolve_ty(primitives),
            ValueTy::Array(array_ty) => {
                let elem_size = array_ty.elem.get_size();
                let elem_align = array_ty.elem.get_align();

                ResolveTy {
                    size: elem_size * array_ty.len,
                    align: elem_align,
                    ty: AnyTy::Value(ValueTy::Array(array_ty)),
                }
            }
            ValueTy::Tuple(mut tuple_ty) => {
                if tuple_ty.fields.is_empty() {
                    return ResolveTy {
                        size: 0,
                        align: 1,
                        ty: AnyTy::Value(ValueTy::Unit),
                    };
                }

                let (size, align) = resolve_fields_size_align(&mut tuple_ty.fields);

                ResolveTy {
                    size,
                    align,
                    ty: AnyTy::Value(ValueTy::Tuple(tuple_ty)),
                }
            }
            ValueTy::Struct(_) => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    ty: AnyTy::Value(self),
                }
            }
            ValueTy::Fn(_) | ValueTy::Ptr(_) | ValueTy::ManyPtr(_) => {
                ResolveTy {
                    size: primitives.ptr_size,
                    align: primitives.ptr_align,
                    ty: AnyTy::Value(self),
                }
            }
            ValueTy::Slice(mut slice_ty) => {
                let (size, align) = resolve_fields_size_align(&mut slice_ty.fields);

                ResolveTy {
                    size,
                    align,
                    ty: AnyTy::Value(self),
                }
            }
        }
    }
}

impl fmt::Display for ValueTy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unit => write!(f, "()"),
            Self::Bool => write!(f, "bool"),
            Self::Char => write!(f, "char"),
            Self::Int(int_ty) => write!(f, "{int_ty}"),
            Self::Float(float_ty) => write!(f, "{float_ty}"),
            Self::Array(array_ty) => write!(f, "{array_ty}"),
            Self::Tuple(tuple_ty) => write!(f, "{tuple_ty}"),
            Self::Struct(struct_ty) => write!(f, "{struct_ty}"),
            Self::Fn(fn_ty) => write!(f, "{fn_ty}"),
            Self::Ptr(ptr_ty) => write!(f, "{ptr_ty}"),
            Self::ManyPtr(many_ptr_ty) => write!(f, "{many_ptr_ty}"),
            Self::Slice(slice_ty) => write!(f, "{slice_ty}"),
        }
    }
}
