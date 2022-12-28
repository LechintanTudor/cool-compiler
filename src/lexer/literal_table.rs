use crate::lexer::Literal;

#[derive(Default)]
pub struct LiteralTable {
    literals: Vec<Literal>,
}

impl LiteralTable {
    pub fn insert(&mut self, literal: Literal) -> usize {
        let index = self.literals.len();
        self.literals.push(literal);
        index
    }

    pub fn get(&self, index: usize) -> Option<&Literal> {
        self.literals.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Literal> {
        self.literals.iter()
    }
}
