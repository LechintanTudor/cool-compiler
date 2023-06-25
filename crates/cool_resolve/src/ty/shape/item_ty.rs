use derive_more::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Debug)]
pub enum ItemTy {
    #[display(fmt = "module")]
    Module,

    #[display(fmt = "type")]
    Ty,
}
