use crate::{ExternFnItem, FnExpr};

#[derive(Clone, Debug)]
pub enum AbstractFn {
    Fn(FnExpr),
    ExternFn(ExternFnItem),
}

impl From<FnExpr> for AbstractFn {
    #[inline]
    fn from(fn_expr: FnExpr) -> Self {
        Self::Fn(fn_expr)
    }
}

impl From<ExternFnItem> for AbstractFn {
    #[inline]
    fn from(extern_fn_item: ExternFnItem) -> Self {
        Self::ExternFn(extern_fn_item)
    }
}
