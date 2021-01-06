#![feature(array_map)]
use quickcheck::TestResult;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[quickcheck_macros::quickcheck]
fn qc_genlut_lut8x16(
    table_row: usize,
    out_row: usize,
    mut indices: Vec<u8>,
    mut values: Vec<u8>,
) -> TestResult {
    values.resize_with(64, u8::default);
    indices.resize_with(32, u8::default);
    let out_row = out_row % 8;
    let table_row = table_row % 8;
    if table_row == 0 {
        return TestResult::discard();
    }

    indices.resize_with(64, u8::default); // extra for loading

    log::debug!("values = {:x?}", values);
    log::debug!("indices = {:x?}", indices);
    log::debug!("table_row = {:x?}", table_row);
    log::debug!("out_row = {:x?}", out_row);

    let mut got = [0u8; 64];
    let all_x;
    unsafe {
        amx::enable();
        amx::load512_x(values.as_ptr(), table_row);
        amx::load512_x(indices.as_ptr(), 0); // TODO: make this dynamic
        amx::ops::op_in::<22>(
            ((out_row as u64) << 20)
                | (1 << 53)
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
            // TODO: suboffset inside the row?
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
