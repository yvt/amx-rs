use amx::{prelude::*, XBytes, XRow, YBytes, YRow, ZRow};
use itertools::iproduct;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

struct Xorshift32(u32);

impl Xorshift32 {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
}

fn read_array_wrapping<T: Copy, const N: usize>(a: &[T], i: usize) -> [T; N] {
    use std::mem::MaybeUninit;
    let mut out = [MaybeUninit::<T>::uninit(); N];
    for j in 0..N {
        out[j] = MaybeUninit::new(a[i.wrapping_add(j) % a.len()]);
    }
    unsafe { std::mem::transmute_copy(&out) }
}

#[test]
fn outer_product_i16_xy_to_z() {
    init();
    unsafe {
        let mut ctx = amx::AmxCtx::new().unwrap();

        let mut rng = Xorshift32(0x114514);
        let in_x: Vec<u8> = (0..512).map(|_| rng.next() as u8).collect();
        let in_y: Vec<u8> = (0..512).map(|_| rng.next() as u8).collect();
        let mut expected_z = ctx.read_z();

        for i in 0..8 {
            ctx.load512(&in_x[i * 64], XRow(i));
            ctx.load512(&in_y[i * 64], YRow(i));
        }

        log::info!("x = {:?}", *(in_x.as_ptr() as *const [[u16; 32]; 8]));
        log::info!("y = {:?}", *(in_y.as_ptr() as *const [[u16; 32]; 8]));

        for (x_offset, y_offset, &z_index) in iproduct!(
            (0..0x200).step_by(31),
            (0..0x200).step_by(47),
            &[0, 1, 50, 63]
        ) {
            log::debug!(
                "(x_offset, y_offset, z_index) = {:?}",
                (x_offset, y_offset, z_index)
            );

            ctx.outer_product_i16_xy_to_z(
                Some(XBytes(x_offset)),
                Some(YBytes(y_offset)),
                ZRow(z_index),
                false, // don't accumulate
            );

            // Calculate the expected answer
            for x_i in (0..64usize).step_by(2) {
                for y_i in (0..64usize).step_by(2) {
                    let x =
                        i16::from_le_bytes(read_array_wrapping(&in_x, x_i.wrapping_add(x_offset)));
                    let y =
                        i16::from_le_bytes(read_array_wrapping(&in_y, y_i.wrapping_add(y_offset)));
                    let prod = x.wrapping_mul(y).to_le_bytes();
                    let out_row = (z_index % 2 + y_i) % 64;
                    expected_z[out_row * 64 + x_i..][..2].copy_from_slice(&prod);
                }
            }

            // Get the actual answer
            let got_z = ctx.read_z();

            assert_eq!(
                std::mem::transmute::<_, [[u16; 32]; 64]>(got_z),
                std::mem::transmute::<_, [[u16; 32]; 64]>(expected_z)
            );
        }
    }
}
