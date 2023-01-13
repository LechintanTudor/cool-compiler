#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct Span {
    pub start: u32,
    pub len: u32,
}

impl Span {
    pub fn new(start: u32, len: u32) -> Self {
        Self { start, len }
    }

    pub fn from_start_and_end(start: u32, end: u32) -> Self {
        Self {
            start,
            len: end - start,
        }
    }
}
