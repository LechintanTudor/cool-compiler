#[derive(Clone, Debug)]
pub struct LineOffsets {
    offsets: Vec<u32>,
}

impl Default for LineOffsets {
    fn default() -> Self {
        Self { offsets: vec![0] }
    }
}

impl LineOffsets {
    pub fn add(&mut self, offset: u32) {
        debug_assert!(*self.offsets.last().unwrap() < offset);

        self.offsets.push(offset)
    }

    pub fn as_slice(&self) -> &[u32] {
        self.offsets.as_slice()
    }

    pub fn find_line_for_offset(&mut self, offset: u32) -> u32 {
        self.offsets
            .partition_point(|&line_offset| line_offset < offset) as u32
    }
}
