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
