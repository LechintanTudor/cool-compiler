use cool_derive::define_tys;

define_tys! {
    // Infer
    infer,
    infer_number,
    infer_int,
    infer_int_or_bool,

    // Item
    alias,
    module,

    // Defined
    unit,
    bool,
    char,

    i8,
    i16,
    i32,
    i64,
    i128,
    isize,

    u8,
    u16,
    u32,
    u64,
    u128,
    usize,

    f32,
    f64,
}
