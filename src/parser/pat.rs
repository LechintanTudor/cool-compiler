#[derive(Clone, Debug)]
pub enum PatAst {
    Ident(IdentPatAst),
    Tuple(TuplePatAst),
    Array(ArrayPatAst),
}

#[derive(Clone, Copy, Debug)]
pub struct IdentPatAst {
    pub is_mutable: bool,
    pub index: u32,
}

#[derive(Clone, Debug)]
pub struct TuplePatAst {
    pub elems: Vec<PatAst>,
}

#[derive(Clone, Debug)]
pub struct ArrayPatAst {
    pub elems: Vec<PatAst>,
}
