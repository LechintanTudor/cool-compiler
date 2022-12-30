use crate::lexer::Literal;

#[derive(Default)]
pub struct LiteralTable {
    literals: Vec<Literal>,
}

impl LiteralTable {
    pub fn insert(&mut self, literal: Literal) -> u32 {
        let index = self.literals.len() as u32;
        self.literals.push(literal);
        index
    }

    pub fn get(&self, index: u32) -> Option<&Literal> {
        self.literals.get(index as usize)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Literal> {
        self.literals.iter()
    }
}
