#[derive(Clone, Default, Debug)]
pub struct LineOffsets {
    offsets: Vec<u32>,
}

impl LineOffsets {
    #[inline]
    pub fn add_line(&mut self, offset: u32) {
        debug_assert!(self.offsets.last().filter(|&&o| o >= offset).is_none());
        self.offsets.push(offset);
    }

    #[inline]
    #[must_use]
    pub fn get_line(&self, offset: u32) -> u32 {
        self.offsets.partition_point(|&o| o <= offset) as u32
    }
}
