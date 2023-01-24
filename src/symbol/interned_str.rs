use std::borrow::Borrow;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct InternedStr {
    start: *const u8,
    len: usize,
}

unsafe impl Send for InternedStr {}
unsafe impl Sync for InternedStr {}

impl fmt::Debug for InternedStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.as_str())
    }
}

impl fmt::Display for InternedStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl InternedStr {
    pub unsafe fn from_str(str: &str) -> Self {
        Self {
            start: str.as_ptr(),
            len: str.len(),
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.start, self.len);
            std::str::from_utf8_unchecked(slice)
        }
    }
}

impl Borrow<str> for InternedStr {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for InternedStr {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for InternedStr {}

impl Hash for InternedStr {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.as_str().hash(state);
    }
}
