#![feature(array_map)]
use quickcheck::TestResult;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[quickcheck_macros::quickcheck]
fn qc_genlut_lut8x16(
    table_row: usize,
    index_offset: usize,
    indices_in_y: bool,
    out_row: usize,
    mut indices: Vec<u8>,
    mut values: Vec<u8>,
) -> TestResult {
    values.resize_with(64, u8::default);
    indices.resize_with(32, u8::default);
    let out_row = out_row % 8;
    let table_row = table_row % 8;
    let index_offset = index_offset % 512;
    if overlaps(
        index_offset..index_offset + 64,
        table_row * 64..table_row * 64 + 64,
    ) || overlaps(
        index_offset..index_offset + 64,
        table_row * 64 + 512..table_row * 64 + 64 + 512,
    ) {
        return TestResult::discard();
    }

    log::debug!("values = {:x?}", values);
    log::debug!("indices = {:x?}", indices);
    log::debug!("table_row = {:x?}", table_row);
    log::debug!("index_offset = {:x?}", index_offset);
    log::debug!("out_row = {:x?}", out_row);
    log::debug!("indices_in_y = {:x?}", indices_in_y);

    let mut got = [0u8; 64];
    let all_x;
    unsafe {
        amx::enable();
        {
            indices.resize_with(64, u8::default);

            // Load `indices` at byte offset `index_offset`
            let mut index_row_1 = [0u8; 64];
            let mut index_row_2 = [0u8; 64];
            let sub = index_offset % 64;
            index_row_1[sub..].copy_from_slice(&indices[..64 - sub]);
            index_row_2[..sub].copy_from_slice(&indices[64 - sub..]);
            if indices_in_y {
                amx::load512_y(index_row_1.as_ptr(), index_offset / 64);
                amx::load512_y(index_row_2.as_ptr(), index_offset / 64 + 1);
            } else {
                amx::load512_x(index_row_1.as_ptr(), index_offset / 64);
                amx::load512_x(index_row_2.as_ptr(), index_offset / 64 + 1);
            }
        }
        amx::load512_x(values.as_ptr(), table_row);
        amx::ops::op_in::<22>(
            (index_offset as u64)
                | ((out_row as u64) << 20)
                // TODO: there's something at bit 25, 26
                | ((indices_in_y as u64) << 10)
                | (1 << 53)
                // TODO: there's something at bit 54
                | (1 << 55)
                | (1 << 56)
                | ((table_row as u64) << 60),
        );
        amx::store512_x(got.as_mut_ptr(), out_row);
        all_x = std::mem::transmute::<_, [[u64; 8]; 8]>(amx::read_x());
        amx::disable();
    }

    let expected: Vec<u8> = (0..64)
        .map(|i| {
            let idx = (indices[i / 2] >> (i % 2 * 4)) as usize & 0xf;
            values[idx]
        })
        .collect();

    log::debug!("got = {:x?}", got);
    log::debug!("expected = {:x?}", expected);
    log::debug!("all_x = {:x?}", all_x);

    assert_eq!(
        got[..],
        expected[..],
        "got = {:?}, expected = {:?}",
        got,
        expected,
    );

    TestResult::passed()
}
