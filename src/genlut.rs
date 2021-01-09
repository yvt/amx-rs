//! Wrapper for the `amxgenlut` instruction
#[derive(Clone, Copy)]
enum Mode {
    ReverseU4IndicesF32Values = 0,
    ReverseU5IndicesF16Values,
    ReverseU4IndicesF64Values,
    ReverseU4IndicesI32Values,
    ReverseU5IndicesI16Values,
    ReverseU4IndicesU32Values,
    ReverseU5IndicesU16Values,
    U2IndicesU32Values,
    U2IndicesU16Values,
    U2IndicesU8Values,
    U4IndicesU64Values,
    U4IndicesU32Values,
    U4IndicesU16Values,
    U4IndicesU8Values,
    U5IndicesU16Values,
    U5IndicesU8Values,
}

/// `output_row` specifies the output row index. If `output_in_z` is `false`, it
/// specifies a row in X (if it's in `0..8`) or Y (if it's in `32..40`).
/// If `output_in_z` is `true`, it specifies a row in Z.
#[inline(always)]
unsafe fn lut_generic(
    input_offset_bytes: usize,
    input_in_y: bool,
    table_x_row: usize,
    output_row: usize,
    output_in_z: bool,
    mode: Mode,
) {
    crate::ops::op_in::<22>(
        (input_offset_bytes as u64)
            | ((input_in_y as u64) << 10)
            | ((output_row as u64) << 20)
            | ((output_in_z as u64) << 26)
            | ((mode as u64) << 53)
            | ((table_x_row as u64) << 60),
    );
}
