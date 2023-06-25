#[derive(Clone, Copy, Debug)]
pub struct PrimitiveTyData {
    pub i8_align: u64,
    pub i16_align: u64,
    pub i32_align: u64,
    pub i64_align: u64,
    pub i128_align: u64,
    pub ptr_size: u64,
    pub ptr_align: u64,
    pub f32_align: u64,
    pub f64_align: u64,
}
