use smallvec::SmallVec;
use std::{fmt, ops};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SmallVecMap<K, V, const N: usize> {
    inner: SmallVec<[(K, V); N]>,
}

impl<K, V, const N: usize> SmallVecMap<K, V, N> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: PartialEq,
    {
        let found_index = self.inner.iter().position(|(k, _)| k == &key);

        match found_index {
            Some(i) => Some(std::mem::replace(&mut self.inner[i], (key, value)).1),
            None => {
                self.inner.push((key, value));
                None
            }
        }
    }

    pub fn insert_if_not_exists(&mut self, key: K, value: V) -> bool
    where
        K: PartialEq,
    {
        if self.contains_key(&key) {
            return false;
        }

        self.inner.push((key, value));
        true
    }

    pub fn insert_unchecked(&mut self, key: K, value: V) {
        self.inner.push((key, value));
    }

    pub fn contains_key(&self, key: &K) -> bool
    where
        K: PartialEq,
    {
        self.inner.iter().any(|(k, _)| k == key)
    }

    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: PartialEq,
    {
        self.inner.iter().find_map(|(k, v)| (k == key).then_some(v))
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V>
    where
        K: PartialEq,
    {
        self.inner
            .iter_mut()
            .find_map(|(k, v)| (k == key).then_some(v))
    }

    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.inner.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.iter().map(|(k, _)| k)
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.inner.iter().map(|(_, v)| v)
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<K, V, const N: usize> Default for SmallVecMap<K, V, N> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<K, V, const N: usize> ops::Index<&K> for SmallVecMap<K, V, N>
where
    K: PartialEq,
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        self.get(key).unwrap()
    }
}

impl<K, V, const N: usize> ops::IndexMut<&K> for SmallVecMap<K, V, N>
where
    K: PartialEq,
{
    fn index_mut(&mut self, key: &K) -> &mut Self::Output {
        self.get_mut(key).unwrap()
    }
}

impl<K, V, const N: usize> fmt::Debug for SmallVecMap<K, V, N>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.inner.iter().map(|(k, v)| (k, v)))
            .finish()
    }
}
