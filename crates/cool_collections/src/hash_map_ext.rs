use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};

pub trait HashMapExt<K, V> {
    fn insert_if_not_exists(&mut self, key: K, value: V) -> Option<&mut V>;
}

impl<K, V, S> HashMapExt<K, V> for HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn insert_if_not_exists(&mut self, key: K, value: V) -> Option<&mut V> {
        match self.entry(key) {
            Entry::Vacant(entry) => Some(entry.insert(value)),
            Entry::Occupied(_) => None,
        }
    }
}
