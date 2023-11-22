mod section;

pub use self::section::*;

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Span {
    pub start: u32,
    pub len: u32,
}

impl Span {
    pub const EMPTY: Self = Self { start: 0, len: 0 };

    pub const OUT_OF_BOUNDS: Self = Self {
        start: u32::MAX,
        len: 0,
    };

    #[inline]
    #[must_use]
    pub const fn from_to(from: u32, to: u32) -> Self {
        debug_assert!(from <= to);

        Self {
            start: from,
            len: to - from,
        }
    }

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

    #[inline]
    #[must_use]
    pub const fn is_out_of_bounds(&self) -> bool {
        self.start == Self::OUT_OF_BOUNDS.start
    }
}

impl fmt::Debug for Span {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end())
    }
}
