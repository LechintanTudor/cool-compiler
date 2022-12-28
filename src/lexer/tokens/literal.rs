#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Literal {
    Integer(String),
    String(String),
}
