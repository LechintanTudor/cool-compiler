#[derive(Clone, Default, Debug)]
pub struct LineOffsets {
    offsets: Vec<u32>,
}

impl LineOffsets {
    #[inline]
    pub fn add_line(&mut self, line_length: u32) {
        let prev_offset = self.offsets.last().copied().unwrap_or(0);
        self.offsets.push(prev_offset + line_length);
    }

    #[inline]
    #[must_use]
    pub fn get_line(&self, offset: u32) -> u32 {
        self.offsets.partition_point(|&o| o <= offset) as u32
    }
}
