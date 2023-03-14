use cool_span::{LineSpan, Span};

#[derive(Clone, Debug)]
pub struct LineOffsets {
    offsets: Vec<u32>,
}

impl Default for LineOffsets {
    #[inline]
    fn default() -> Self {
        Self { offsets: vec![0] }
    }
}

impl LineOffsets {
    #[inline]
    pub fn clear(&mut self) {
        self.offsets.drain(1..);
    }

    #[inline]
    pub fn add(&mut self, offset: u32) {
        debug_assert!(*self.offsets.last().unwrap() < offset);
        self.offsets.push(offset)
    }

    #[inline]
    pub fn as_slice(&self) -> &[u32] {
        self.offsets.as_slice()
    }

    #[inline]
    pub fn to_line(&self, offset: u32) -> u32 {
        self.offsets
            .partition_point(|&line_offset| line_offset <= offset) as u32
    }

    #[inline]
    pub fn to_line_span(&self, span: Span) -> LineSpan {
        let start = self.to_line(span.start);
        let end = self.to_line(span.end());
        LineSpan::from_to(start, end)
    }
}
