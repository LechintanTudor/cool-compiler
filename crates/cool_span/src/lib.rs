mod section;

pub use self::section::*;

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Span {
    pub start: u32,
    pub len: u32,
}

impl Span {
    #[inline]
    #[must_use]
    pub const fn to(&self, end_span: Span) -> Span {
        Self {
            start: self.start,
            len: end_span.end() - self.start,
        }
    }

    #[inline]
    #[must_use]
    pub const fn end(&self) -> u32 {
        self.start + self.len
    }
}

impl fmt::Debug for Span {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end())
    }
}
