#[derive(Clone, Debug)]
pub struct FnArgAst {
    pub is_mutable: bool,
    pub ident_index: u32,
    pub type_ident_index: u32,
}

#[derive(Clone, Debug)]
pub struct FnArgListAst {
    pub args: Vec<FnArgAst>,
}

#[derive(Clone, Debug)]
pub struct FnAst {
    pub args: FnArgListAst,
}
