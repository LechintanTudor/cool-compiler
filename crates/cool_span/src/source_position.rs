#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct SourcePosition {
    pub line: u32,
    pub column: u32,
}
