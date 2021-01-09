#![feature(array_chunks)]
use aligned_box::AlignedBox;
use amx::{prelude::*, AmxOps, XRow, YRow, ZRow};
use itertools::iproduct;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MemSize {
    /// 64 bytes
    _64 = 0,
    /// 128 bytes
    _128 = 1,
}

impl MemSize {
    pub fn num_bytes(self) -> usize {
        match self {
            Self::_64 => 64,
            Self::_128 => 128,
        }
    }
}

unsafe fn load_generic<T>(
    ctx: &mut impl AmxOps,
    ptr: *const T,
    index: usize,
    size: MemSize,
    reg: u8,
    interleaved: bool,
) {
    match (size, reg, interleaved) {
        (MemSize::_64, 0, false) => {
            ctx.load512(ptr, XRow(index));
        }
        (MemSize::_64, 1, false) => {
            ctx.load512(ptr, YRow(index));
        }
        (MemSize::_64, 2, false) => {
            ctx.load512(ptr, ZRow(index));
        }
        (MemSize::_64, 2, true) => {
            ctx.load512_interleaved(ptr, ZRow(index));
        }
        (MemSize::_128, 0, false) => {
            ctx.load1024_aligned(ptr, XRow(index));
        }
        (MemSize::_128, 1, false) => {
            ctx.load1024_aligned(ptr, YRow(index));
        }
        (MemSize::_128, 2, false) => {
            ctx.load1024_aligned(ptr, ZRow(index));
        }
        _ => unreachable!(),
    }
}

unsafe fn store_generic<T>(
    ctx: &mut impl AmxOps,
    ptr: *mut T,
    index: usize,
    size: MemSize,
    reg: u8,
    interleaved: bool,
) {
    match (size, reg, interleaved) {
        (MemSize::_64, 0, false) => {
            ctx.store512(ptr, XRow(index));
        }
        (MemSize::_64, 1, false) => {
            ctx.store512(ptr, YRow(index));
        }
        (MemSize::_64, 2, false) => {
            ctx.store512(ptr, ZRow(index));
        }
        (MemSize::_64, 2, true) => {
            ctx.store512_interleaved(ptr, ZRow(index));
        }
        (MemSize::_128, 0, false) => {
            ctx.store1024_aligned(ptr, XRow(index));
        }
        (MemSize::_128, 1, false) => {
            ctx.store1024_aligned(ptr, YRow(index));
        }
        (MemSize::_128, 2, false) => {
            ctx.store1024_aligned(ptr, ZRow(index));
        }
        _ => unreachable!(),
    }
}

#[test]
fn copy_and_check_memory() {
    init();
    let mut ctx = amx::AmxCtx::new().unwrap();

    let mut src: AlignedBox<[u16]> = AlignedBox::slice_from_default(0x80, 4096).unwrap();
    for (i, src) in src.iter_mut().enumerate() {
        *src = i as _;
    }

    for (&size, &reg, reg_offset, &interleaved) in iproduct!(
        &[MemSize::_64, MemSize::_128],
        &[0, 1, 2],
        0..64,
        &[false, true]
    ) {
        if interleaved && (reg != 2 || size != MemSize::_64) {
            continue;
        }
        if reg != 2 && reg_offset >= 8 {
            continue;
        }

        log::debug!(
            "size = {:?}, reg = amx{}, reg_offset = {}, interleaved = {}",
            size,
            reg,
            reg_offset,
            interleaved
        );

        let mut got: AlignedBox<[u16]> = AlignedBox::slice_from_value(0x80, 4096, 0xbeef).unwrap();
        let expected: Vec<u16> = (0..4096)
            .map(|i| {
                if i as usize * 2 < size.num_bytes() {
                    i
                } else {
                    0xbeef
                }
            })
            .collect();

        unsafe {
            load_generic(&mut *ctx, src.as_ptr(), reg_offset, size, reg, interleaved);
            store_generic(
                &mut *ctx,
                got.as_mut_ptr(),
                reg_offset,
                size,
                reg,
                interleaved,
            );
        }

        assert_eq!(*got, *expected);
    }
}

#[test]
fn load_and_check_register() {
    init();
    let mut ctx = amx::AmxCtx::new().unwrap();

    let mut pat1: AlignedBox<[u64]> = AlignedBox::slice_from_default(0x80, 16).unwrap();
    for (i, pat1) in pat1.iter_mut().enumerate() {
        *pat1 = i as u64 + (75 - i as u64) * 0x100000000;
    }

    let pat2: Vec<u64> = vec![0x2222_2222_2222_2222; 512];

    for (&size, &reg, reg_offset, &interleaved) in iproduct!(
        &[MemSize::_64, MemSize::_128],
        &[0, 1, 2],
        0..64,
        &[false, true]
    ) {
        if interleaved && (reg != 2 || size != MemSize::_64) {
            continue;
        }
        if reg != 2 && reg_offset >= 8 {
            continue;
        }

        log::debug!(
            "size = {:?}, reg = amx{}, reg_offset = {}, interleaved = {}",
            size,
            reg,
            reg_offset,
            interleaved
        );

        // Number of `u64`s in the register set
        let reg_size = [64, 64, 512][reg as usize];

        // Fill the register set with `pat2`.
        for i in 0..reg_size / 8 {
            unsafe {
                load_generic(
                    &mut *ctx,
                    pat2[i * 8..].as_ptr() as *mut (),
                    i,
                    MemSize::_64,
                    reg,
                    false,
                );
            }
        }

        // Load `pat1` to somewhere in the register
        unsafe {
            load_generic(&mut *ctx, pat1.as_ptr(), reg_offset, size, reg, interleaved);
        }

        // Read the whole register set
        let got = match reg {
            0 => ctx.read_x().to_vec(),
            1 => ctx.read_y().to_vec(),
            2 => ctx.read_z().to_vec(),
            _ => unreachable!(),
        };
        let got: Vec<u64> = got
            .array_chunks::<8>()
            .map(|x| u64::from_le_bytes(*x))
            .collect();

        // Calculate the expected result
        let mut expected: Vec<u64> = pat2[0..reg_size].to_owned();
        if interleaved {
            // Assume the structure `z: [[u8; 64]; 64]`
            //
            // reg_offset is split into two parts:
            //
            //  - `reg_index = reg_offset / 2 * 2`
            //  - `second_half = reg_offset % 2`.
            //
            // Each input 64-bit value is split into low and high parts, and
            // the resultant low parts go to
            // `z[reg_index][second_half * 4..][..4]`. The high parts go to
            // `z[reg_index + 1][second_half * 4..][..4]`
            let reg_start = (reg_offset as usize % 2) * 4 + (reg_offset as usize / 2) * 16;
            for i in (0..size.num_bytes() / 8).step_by(2) {
                let low1 = pat1[i] & 0xffff_ffff;
                let low2 = pat1[i + 1] & 0xffff_ffff;
                let high1 = pat1[i] >> 32;
                let high2 = pat1[i + 1] >> 32;
                expected[(reg_start + i / 2) % reg_size] = low1 | (low2 << 32);
                expected[(reg_start + 8 + i / 2) % reg_size] = high1 | (high2 << 32);
            }
        } else {
            // Simple copy with register index wrap-around
            for i in 0..size.num_bytes() / 8 {
                expected[(reg_offset as usize * 8 + i) % reg_size] = pat1[i];
            }
        }

        assert_eq!(
            got, expected,
            "\ngot = {:x?}\nexpected = {:x?}",
            got, expected
        );
    }
}
