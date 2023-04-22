use crate::{ExternFnAst, FnAst};

#[derive(Clone, Debug)]
pub struct ProgramAst {
    pub fns: Vec<FnAst>,
    pub extern_fns: Vec<ExternFnAst>,
}
