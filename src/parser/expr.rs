use crate::parser::Parser;

#[derive(Clone, Debug)]
pub enum ExprAst {
    Ident(IdentExprAst),
    Lit(LitExprAst),
    // Tuple(TupleExprAst),
    // Array(ArrayExprAst),
    // Parens(Box<ExprAst>),
    // UnaryOp{ op: UnaryOp, expr: Box<ExprAst> },
    // BinOp { left: Box<ExprAst>, op: BinOp, right: Box<ExprAst> },
}

#[derive(Clone, Debug)]
pub struct IdentExprAst {
    pub index: u32,
}

#[derive(Clone, Debug)]
pub struct LitExprAst {
    pub index: u32,
}

#[derive(Clone, Debug)]
pub struct TupleExprAst {
    pub elems: Vec<ExprAst>,
}

#[derive(Clone, Debug)]
pub struct ArrayExprAst {
    pub elems: Vec<ExprAst>,
}

impl Parser<'_> {
    pub fn parse_expr(&mut self) -> anyhow::Result<ExprAst> {
        if let Some(index) = self.consume_ident() {
            Ok(ExprAst::Ident(IdentExprAst { index }))
        } else if let Some(index) = self.consume_lit() {
            Ok(ExprAst::Lit(LitExprAst { index }))
        } else {
            panic!("not yet supported")
        }
    }
}
