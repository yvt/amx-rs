//! Rust wrapper for Apple Matrix Coprocessor (AMX) instructions
//!
//! This crate provides wrapper functions for the undocumented AMX instructions,
//! which are found in Apple Silicon processors.
//!
//! # Resources
//!
//!  - <https://gist.github.com/dougallj/7a75a3be1ec69ca550e7c36dc75e0d6f>
//!  - <https://www.realworldtech.com/forum/?threadid=187087&curpostid=187120>
//!
//! # Example
//!
//! ```rust
//! use amx::Amx;
//! let mut ctx = amx::AmxCtx::new().unwrap();
//! let x = [1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15, 16,
//!          17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32i16];
//! let y = [51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66,
//!          67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82i16];
//! unsafe { ctx.load512_x(x.as_ptr(), 0) };
//! unsafe { ctx.load512_y(y.as_ptr(), 0) };
//! ctx.outer_product_i16_xy_to_z(
//!     0,     // input from X starting from byte offset 0
//!     0,     // input from Y starting from byte offset 0
//!     0,     // output to Z starting from row offset 0
//!     false, // don't accumulate
//!     false, // use X
//!     false, // use Y
//! );
//! let z: [[i16; 32]; 64] = unsafe { std::mem::transmute(ctx.read_z()) };
//! for (x_i, &x) in x.iter().enumerate() {
//!     for (y_i, &y) in y.iter().enumerate() {
//!         assert_eq!(z[y_i * 2][x_i], x * y);
//!     }
//! }
//! ```
//!
//! # Registers
//!
//! ```rust
//! struct AmxState {
//!     /// "8 64-byte registers"
//!     x: [[u8; 64]; 8],
//!     /// "8 64-byte registers"
//!     y: [[u8; 64]; 8],
//!     /// "64 64-byte registers in an M-by-N matrix"
//!     z: [[u8; 64]; 64],
//! }
//! ```
#![feature(asm)]

// TODO: mod genlut;
mod nativectx;
pub mod nativeops;
mod ops;
pub use crate::{
    nativectx::{AmxCtx, NewAmxCtxError},
    ops::AmxOps,
};

/// The prelude.
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::{ops::AmxOps as _, Amx as _};
}

/// A high-level wrapper for AMX instructions.
pub trait Amx: crate::ops::AmxOps {
    /// Load 512 bits (64 bytes) from memory to `x[index % 8][0..64]`.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn load512_x<T>(&mut self, ptr: *const T, index: usize) {
        debug_assert!(index < 64);
        self.ldx(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
        );
    }

    /// Load 512 bits (64 bytes) from memory to `y[index % 8][0..64]`.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn load512_y<T>(&mut self, ptr: *const T, index: usize) {
        debug_assert!(index < 64);
        self.ldy(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
        );
    }

    /// Load 512 bits (64 bytes) from memory to `z[index][0..64]`.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn load512_z<T>(&mut self, ptr: *const T, index: usize) {
        debug_assert!(index < 64);
        self.ldz(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
        );
    }

    /// Load 512 bits (64 bytes) from memory to `z[index][0..64]` with interleaving.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn load512_z_interleaved<T>(&mut self, ptr: *const T, index: usize) {
        debug_assert!(index < 64);
        self.ldzi(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
        );
    }

    /// Load 1024 bits (128 bytes) from memory to
    /// `[x[index % 8][0..64], x[(index + 1) % 8][0..64]]`.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn load1024_x_aligned<T>(&mut self, ptr: *const T, index: usize) {
        debug_assert!(index < 64);
        self.ldx(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
        );
    }

    /// Load 1024 bits (128 bytes) from memory to
    /// `[y[index % 8][0..64], y[(index + 1) % 8][0..64]]`.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn load1024_y_aligned<T>(&mut self, ptr: *const T, index: usize) {
        debug_assert!(index < 64);
        self.ldy(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
        );
    }

    /// Load 1024 bits (128 bytes) from memory to
    /// `[z[index][0..64], z[(index + 1) % 64][0..64]]`.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn load1024_z_aligned<T>(&mut self, ptr: *const T, index: usize) {
        debug_assert!(index < 64);
        self.ldz(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
        );
    }

    /// Store 512 bits (64 bytes) `x[index % 8][0..64]` to memory.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn store512_x<T>(&mut self, ptr: *mut T, index: usize) {
        debug_assert!(index < 64);
        self.stx(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
        );
    }

    /// Store 512 bits (64 bytes) `y[index % 8][0..64]` to memory.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn store512_y<T>(&mut self, ptr: *mut T, index: usize) {
        debug_assert!(index < 64);
        self.sty(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
        );
    }

    /// Store 512 bits (64 bytes) `z[index][0..64]` to memory.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn store512_z<T>(&mut self, ptr: *mut T, index: usize) {
        debug_assert!(index < 64);
        self.stz(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
        );
    }

    /// Store 512 bits (64 bytes) `z[index][0..64]` to memory with interleaving.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn store512_z_interleaved<T>(&mut self, ptr: *mut T, index: usize) {
        debug_assert!(index < 64);
        self.stzi(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
        );
    }

    /// Store 1024 bits (128 bytes to memory)
    /// `[x[index % 8][0..64], x[(index + 1) % 8][0..64]]`.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn store1024_x_aligned<T>(&mut self, ptr: *mut T, index: usize) {
        debug_assert!(index < 64);
        self.stx(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
        );
    }

    /// Store 1024 bits (128 bytes to memory)
    /// `[y[index % 8][0..64], y[(index + 1) % 8][0..64]]`.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn store1024_y_aligned<T>(&mut self, ptr: *mut T, index: usize) {
        debug_assert!(index < 64);
        self.sty(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
        );
    }

    /// Store 1024 bits (128 bytes to memory)
    /// `[z[index][0..64], z[(index + 1) % 64][0..64]]`.
    ///
    /// `index` must be in range `0..64`.
    #[inline(always)]
    unsafe fn store1024_z_aligned<T>(&mut self, ptr: *mut T, index: usize) {
        debug_assert!(index < 64);
        self.stz(
            MemArgs {
                ptr: ptr as *mut (),
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
        );
    }

    /// Read the whole contents of `x`.
    fn read_x(&mut self) -> [u8; 512] {
        let mut ret = std::mem::MaybeUninit::uninit();
        for i in 0..8 {
            // Safety: Writing in a memory region within `ret`
            unsafe { self.store512_x((ret.as_mut_ptr() as *mut u8).offset(i as isize * 64), i) };
        }
        // Safety: All elements are initialized
        unsafe { ret.assume_init() }
    }

    /// Read the whole contents of `y`.
    fn read_y(&mut self) -> [u8; 512] {
        let mut ret = std::mem::MaybeUninit::uninit();
        for i in 0..8 {
            // Safety: Writing in a memory region within `ret`
            unsafe { self.store512_y((ret.as_mut_ptr() as *mut u8).offset(i as isize * 64), i) };
        }
        // Safety: All elements are initialized
        unsafe { ret.assume_init() }
    }

    /// Read the whole contents of `z`.
    fn read_z(&mut self) -> [u8; 4096] {
        let mut ret = std::mem::MaybeUninit::uninit();
        for i in 0..64 {
            // Safety: Writing in a memory region within `ret`
            unsafe { self.store512_z((ret.as_mut_ptr() as *mut u8).offset(i as isize * 64), i) };
        }
        // Safety: All elements are initialized
        unsafe { ret.assume_init() }
    }

    /// Calculate the outer product of `x: [i16; 32]` and `y: [i16; 32]` and write
    /// the output to every second row of `z: [[i16; 32]; 64]`.
    ///
    /// `z_index` must be in range `0..64`. Only the least significant bit of
    /// `z_index` will be taken into consideration.
    #[inline(always)]
    fn outer_product_i16_xy_to_z(
        &mut self,
        x_offset_bytes: usize,
        y_offset_bytes: usize,
        z_index: usize,
        accumulate: bool,
        ignore_x: bool,
        ignore_y: bool,
    ) {
        debug_assert!(x_offset_bytes < 0x200);
        debug_assert!(y_offset_bytes < 0x200);
        debug_assert!(z_index < 64);
        // TODO: widening (i32 output)
        // TODO: vector output (reducing)
        self.mac16(
            (y_offset_bytes
                | (x_offset_bytes << 10)
                | (z_index << 20)
                | (((!accumulate) as usize) << 27)
                | ((ignore_x as usize) << 28)
                | ((ignore_y as usize) << 29)) as u64,
        );
    }
}

impl<T: AmxOps + ?Sized> Amx for T {}

/// The parameters of AMX's load and store instructions.
#[derive(Copy, Clone)]
struct MemArgs {
    ptr: *mut (),
    /// 6-bit register offset (in units of `0x40`) in range `0..64`
    reg_offset: u64,
    size: MemSize,
}

impl MemArgs {
    #[inline]
    fn encode(self) -> u64 {
        debug_assert!(self.reg_offset < 64);

        (self.ptr as u64) & 0x00ff_ffff_ffff_ffff
            | (self.reg_offset << 56)
            // [61] - ?
            | ((self.size as u64) << 62)
        // [63] - ?
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum MemSize {
    /// 64 bytes
    _64 = 0,
    /// 128 bytes
    _128 = 1,
}
