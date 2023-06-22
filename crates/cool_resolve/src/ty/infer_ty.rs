use crate::{AnyTy, PrimitiveTyData, ResolveTy};
use derive_more::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Debug)]
pub enum InferTy {
    #[display(fmt = "<any>")]
    Any,

    #[display(fmt = "<int>")]
    Int,

    #[display(fmt = "<float>")]
    Float,

    #[display(fmt = "<number>")]
    Number,

    #[display(fmt = "<array>")]
    Array,

    #[display(fmt = "<ptr>")]
    Ptr,

    #[display(fmt = "<manyptr>")]
    ManyPtr,

    #[display(fmt = "<slice>")]
    Slice,
}

impl InferTy {
    pub fn to_resolve_ty(&self, primitives: &PrimitiveTyData) -> ResolveTy {
        match self {
            Self::Ptr | Self::ManyPtr => {
                ResolveTy {
                    size: primitives.ptr_size,
                    align: primitives.ptr_align,
                    ty: AnyTy::Infer(*self),
                }
            }
            Self::Slice => {
                ResolveTy {
                    size: primitives.ptr_size * 2,
                    align: primitives.ptr_align,
                    ty: AnyTy::Infer(*self),
                }
            }
            _ => {
                ResolveTy {
                    size: 0,
                    align: 1,
                    ty: AnyTy::Infer(*self),
                }
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn has_known_layout(&self) -> bool {
        matches!(self, Self::Ptr | Self::ManyPtr | Self::Slice)
    }
}
