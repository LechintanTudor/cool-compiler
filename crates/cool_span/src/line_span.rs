#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct LineSpan {
    pub start: u32,
    pub len: u32,
}

impl LineSpan {
    pub const fn empty() -> Self {
        Self { start: 0, len: 0 }
    }

    pub const fn new(start: u32, len: u32) -> Self {
        Self { start, len }
    }

    pub const fn from_to(start: u32, end: u32) -> Self {
        Self {
            start,
            len: end - start,
        }
    }

    pub const fn to(&self, end_span: Self) -> Self {
        Self::from_to(self.start, end_span.end())
    }

    pub const fn end(&self) -> u32 {
        self.start + self.len
    }
}
