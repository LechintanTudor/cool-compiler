use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Span {
    pub start: u32,
    pub len: u32,
}

impl Span {
    #[inline]
    pub const fn empty() -> Self {
        Self { start: 0, len: 0 }
    }

    #[inline]
    pub const fn new(start: u32, len: u32) -> Self {
        Self { start, len }
    }

    #[inline]
    pub const fn from_to(start: u32, end: u32) -> Self {
        Self {
            start,
            len: end - start,
        }
    }

    #[inline]
    pub const fn to(&self, end_span: Self) -> Self {
        Self::from_to(self.start, end_span.end())
    }

    #[inline]
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
