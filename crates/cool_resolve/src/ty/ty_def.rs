#[derive(Clone, Debug)]
pub struct TyDef {
    pub size: u64,
    pub align: u64,
    pub kind: TyDefKind,
}

impl TyDef {
    #[inline]
    pub const fn basic(size: u64) -> Self {
        Self {
            size,
            align: size,
            kind: TyDefKind::Basic,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TyDefKind {
    Basic,
}
