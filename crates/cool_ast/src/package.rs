use crate::{DeferStmtMap, ExternFnAst, FnAst};

#[derive(Clone, Debug)]
pub struct PackageAst {
    pub fns: Vec<FnAst>,
    pub extern_fns: Vec<ExternFnAst>,
    pub defer_stmts: DeferStmtMap,
}
