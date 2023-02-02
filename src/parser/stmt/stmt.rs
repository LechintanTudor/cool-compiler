use crate::parser::DeclStmt;
#[derive(Clone, Debug)]
pub enum Stmt {
    Decl(DeclStmt),
}