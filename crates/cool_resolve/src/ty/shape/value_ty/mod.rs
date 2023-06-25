mod array_ty;
mod enum_ty;
mod fn_ty;
mod primitive_ty;
mod ptr_ty;
mod struct_ty;
mod tuple_ty;
mod variant_ty;

pub use self::array_ty::*;
pub use self::enum_ty::*;
pub use self::fn_ty::*;
pub use self::primitive_ty::*;
pub use self::ptr_ty::*;
pub use self::struct_ty::*;
pub use self::tuple_ty::*;
pub use self::variant_ty::*;
use crate::TyShape;
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

                    #[inline]
                    pub fn [<get_ $WrappedTy:snake:lower>](&self) -> &[<$WrappedTy Ty>] {
                        match self {
                            Self::$WrappedTy(ty) => ty,
                            _ => panic!("wrong type"),
                        }
                    }
                )*
            }

            impl TyShape {
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

                    #[inline]
                    pub fn [<get_ $WrappedTy:snake:lower>](&self) -> &[<$WrappedTy Ty>] {
                        match self {
                            Self::Value(ValueTy::$WrappedTy(ty)) => ty,
                            _ => panic!("wrong type"),
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
}
