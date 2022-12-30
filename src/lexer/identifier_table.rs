use std::collections::HashMap;
use std::rc::Rc;

pub struct IdentifierTable {
    identifier_to_index: HashMap<Rc<str>, u32>,
    identifiers: Vec<Rc<str>>,
}

impl Default for IdentifierTable {
    fn default() -> Self {
        let mut table = Self {
            identifier_to_index: Default::default(),
            identifiers: Default::default(),
        };

        table.insert("u8");
        table.insert("i8");
        table.insert("u16");
        table.insert("i16");
        table.insert("u32");
        table.insert("i32");
        table.insert("u64");
        table.insert("i64");
        table.insert("f32");
        table.insert("f64");
        table.insert("bool");
        table.insert("char");

        table
    }
}

impl IdentifierTable {
    pub fn insert(&mut self, identifier: &str) -> u32 {
        if let Some(&index) = self.identifier_to_index.get(identifier) {
            return index;
        }

        let identifier = Rc::<str>::from(identifier);
        let index = self.identifiers.len() as u32;

        self.identifiers.push(Rc::clone(&identifier));
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
