use crate::SmallVec;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Borrow;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Clone, Default)]
pub struct SmallString<const N: usize = 16> {
    data: SmallVec<u8, N>,
}

impl SmallString<16> {
    pub const fn new() -> Self {
        Self {
            data: SmallVec::new_const(),
        }
    }
}

impl<const N: usize> SmallString<N> {
    pub fn push(&mut self, c: char) {
        if c.len_utf8() == 1 {
            self.data.push(c as u8);
        } else {
            self.data
                .extend_from_slice(c.encode_utf8(&mut [4; 0]).as_bytes());
        }
    }

    pub fn push_str(&mut self, str: &str) {
        self.data.extend_from_slice(str.as_bytes())
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.data) }
    }
}

impl<const N: usize, const M: usize> PartialEq<SmallString<M>> for SmallString<N> {
    fn eq(&self, other: &SmallString<M>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<const N: usize> Eq for SmallString<N> {
    // Empty
}

impl<const N: usize> PartialOrd for SmallString<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Ord for SmallString<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<const N: usize> Hash for SmallString<N> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.as_str().hash(state);
    }
}

impl<const N: usize> Borrow<str> for SmallString<N> {
    #[must_use]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> Deref for SmallString<N> {
    type Target = str;

    #[must_use]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const N: usize> From<&str> for SmallString<N> {
    fn from(value: &str) -> Self {
        Self {
            data: SmallVec::from_slice(value.as_bytes()),
        }
    }
}

impl<const N: usize> fmt::Write for SmallString<N> {
    fn write_str(&mut self, str: &str) -> fmt::Result {
        self.data.extend_from_slice(str.as_bytes());
        Ok(())
    }
}

impl<const N: usize> fmt::Debug for SmallString<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.as_str())
    }
}

impl<const N: usize> fmt::Display for SmallString<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'a, const N: usize> Deserialize<'a> for SmallString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        let data = String::deserialize(deserializer)?;
        Ok(SmallString::from(data.as_str()))
    }
}

impl<const N: usize> Serialize for SmallString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
