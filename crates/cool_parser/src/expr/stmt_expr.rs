use crate::StmtKind;
use cool_span::{Section, Span};

#[derive(Clone, Debug)]
pub struct StmtExpr {
    pub stmt: Box<StmtKind>,
}

impl Section for StmtExpr {
    #[inline]
    fn span(&self) -> Span {
        self.stmt.span()
    }
}
