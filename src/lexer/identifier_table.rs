use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
pub struct IdentifierTable {
    identifier_to_index: HashMap<Rc<str>, usize>,
    identifiers: Vec<Rc<str>>,
}

impl IdentifierTable {
    pub fn insert(&mut self, identifier: &str) -> usize {
        if let Some(&index) = self.identifier_to_index.get(identifier) {
            return index;
        }

        let identifier = Rc::<str>::from(identifier);
        let index = self.identifiers.len();

        self.identifiers.insert(index, Rc::clone(&identifier));
        self.identifier_to_index.insert(identifier, index);

        index
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.identifiers
            .get(index)
            .map(|identifier| identifier.as_ref())
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.identifiers.iter().map(Rc::as_ref)
    }
}
