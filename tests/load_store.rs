#![feature(asm)]
use aligned_box::AlignedBox;
use amx::{MemArgs, MemPayload, MemSize};
use itertools::iproduct;

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

unsafe fn load_generic(args: impl Into<MemPayload>, reg: u8, interleaved: bool) {
    match (reg, interleaved) {
        (0, false) => {
            amx::load_amx0(args);
        }
        (1, false) => {
            amx::load_amx1(args);
        }
        (2, false) => {
            amx::load_amx2(args);
        }
        (2, true) => {
            amx::load_amx2_interleaved(args);
        }
        _ => unreachable!(),
    }
}

unsafe fn store_generic(args: impl Into<MemPayload>, reg: u8, interleaved: bool) {
    match (reg, interleaved) {
        (0, false) => {
            amx::store_amx0(args);
        }
        (1, false) => {
            amx::store_amx1(args);
        }
        (2, false) => {
            amx::store_amx2(args);
        }
        (2, true) => {
            amx::store_amx2_interleaved(args);
        }
        _ => unreachable!(),
    }
}

#[test]
fn copy_and_check_memory() {
    init();
    amx::enable();

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
        if interleaved && reg != 2 {
            continue;
        }

        log::debug!(
            "size = {:?}, reg = amx{}, reg_offset = {}, interleaved = {}",
            size,
            reg,
            reg_offset,
            interleaved
        );

        // amxldzi and amxstzi ignores the size bit
        let effective_size = if interleaved { MemSize::_64 } else { size };

        let mut got: AlignedBox<[u16]> = AlignedBox::slice_from_value(0x80, 4096, 0xbeef).unwrap();
        let expected: Vec<u16> = (0..4096)
            .map(|i| {
                if i as usize * 2 < effective_size.num_bytes() {
                    i
                } else {
                    0xbeef
                }
            })
            .collect();

        unsafe {
            load_generic(
                MemArgs {
                    ptr: src.as_ptr() as *mut (),
                    reg_offset,
                    size,
                },
                reg,
                interleaved,
            );
            store_generic(
                MemArgs {
                    ptr: got.as_mut_ptr() as *mut (),
                    reg_offset,
                    size,
                },
                reg,
                interleaved,
            );
        }

        assert_eq!(*got, *expected);
    }

    unsafe { amx::disable() };
}

#[test]
fn load_and_check_register() {
    init();
    amx::enable();

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
        if interleaved && reg != 2 {
            continue;
        }

        log::debug!(
            "size = {:?}, reg = amx{}, reg_offset = {}, interleaved = {}",
            size,
            reg,
            reg_offset,
            interleaved
        );

        // amxldzi and amxstzi ignores the size bit
        let effective_size = if interleaved { MemSize::_64 } else { size };

        // Number of `u64`s in the register set
        let reg_size = [64, 64, 512][reg as usize];

        // Fill the register set with `pat2`.
        for i in 0..reg_size / 8 {
            unsafe {
                load_generic(
                    MemArgs {
                        ptr: pat2[i * 8..].as_ptr() as *mut (),
                        reg_offset: i as _,
                        size: MemSize::_64,
                    },
                    reg,
                    false,
                );
            }
        }

        // Load `pat1` to somewhere in the register
        unsafe {
            load_generic(
                MemArgs {
                    ptr: pat1.as_ptr() as *mut (),
                    reg_offset,
                    size,
                },
                reg,
                interleaved,
            );
        }

        // Read the whole register set
        let mut got: Vec<u64> = vec![0x1111_1111_1111_1111; reg_size];
        for i in 0..reg_size / 8 {
            unsafe {
                store_generic(
                    MemArgs {
                        ptr: got[i * 8..].as_mut_ptr() as *mut (),
                        reg_offset: i as _,
                        size: MemSize::_64,
                    },
                    reg,
                    false,
                );
            }
        }

        // Calculate the expected result
        let mut expected: Vec<u64> = pat2[0..reg_size].to_owned();
        if interleaved {
            // Assume the structure `amx2: [[u8; 64]; 64]`
            //
            // reg_offset is split into two parts:
            //
            //  - `reg_index = reg_offset / 2 * 2`
            //  - `second_half = reg_offset % 2`.
            //
            // Each input 64-bit value is split into low and high parts, and
            // the resultant low parts go to
            // `amx2[reg_index][second_half * 4..][..4]`. The high parts go to
            // `amx2[reg_index + 1][second_half * 4..][..4]`
            let reg_start = (reg_offset as usize % 2) * 4 + (reg_offset as usize / 2) * 16;
            for i in (0..effective_size.num_bytes() / 8).step_by(2) {
                let low1 = pat1[i] & 0xffff_ffff;
                let low2 = pat1[i + 1] & 0xffff_ffff;
                let high1 = pat1[i] >> 32;
                let high2 = pat1[i + 1] >> 32;
                expected[(reg_start + i / 2) % reg_size] = low1 | (low2 << 32);
                expected[(reg_start + 8 + i / 2) % reg_size] = high1 | (high2 << 32);
            }
        } else {
            // Simple copy with register index wrap-around
            for i in 0..effective_size.num_bytes() / 8 {
                expected[(reg_offset as usize * 8 + i) % reg_size] = pat1[i];
            }
        }

        assert_eq!(
            got, expected,
            "\ngot = {:x?}\nexpected = {:x?}",
            got, expected
        );
    }

    unsafe { amx::disable() };
}
