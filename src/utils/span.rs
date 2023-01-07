#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct Span {
    pub start: u32,
    pub len: u32,
}
