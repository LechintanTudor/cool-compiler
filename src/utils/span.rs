#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct Span {
    pub start: u32,
    pub len: u32,
}

impl Span {
    pub const fn empty() -> Self {
        Self { start: 0, len: 0 }
    }

    pub fn new(start: u32, len: u32) -> Self {
        Self { start, len }
    }

    pub fn from_to(start: u32, end: u32) -> Self {
        Self {
            start,
            len: end - start,
        }
    }

    pub fn to(&self, end_span: Span) -> Self {
        Self::from_to(self.start, end_span.end())
    }

    pub fn end(&self) -> u32 {
        self.start + self.len
    }
}
