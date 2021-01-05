//! AMX: Apple Matrix coprocessor
//!
//! This crate provides wrapper functions for the undocumented AMX instructions,
//! which are found in Apple Silicon processors.
//!
//! # Resources
//!
//!  - <https://gist.github.com/dougallj/7a75a3be1ec69ca550e7c36dc75e0d6f>
//!  - <https://www.realworldtech.com/forum/?threadid=187087&curpostid=187120>
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

pub mod ops;

/// Enable the coprocessor.
#[inline(always)]
pub unsafe fn enable() {
    ops::set();
}

/// Disable the coprocessor.
#[inline(always)]
pub unsafe fn disable() {
    ops::clr();
}

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

/// Load 512 bits (64 bytes) from memory to `x[index % 8][0..64]`.
///
/// `index` must be in range `0..64`.
#[inline(always)]
pub unsafe fn load512_x<T>(ptr: *const T, index: usize) {
    debug_assert!(index < 64);
    ops::ldx(
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
pub unsafe fn load512_y<T>(ptr: *const T, index: usize) {
    debug_assert!(index < 64);
    ops::ldy(
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
pub unsafe fn load512_z<T>(ptr: *const T, index: usize) {
    debug_assert!(index < 64);
    ops::ldz(
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
pub unsafe fn load512_z_interleaved<T>(ptr: *const T, index: usize) {
    debug_assert!(index < 64);
    ops::ldzi(
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
pub unsafe fn load1024_x_aligned<T>(ptr: *const T, index: usize) {
    debug_assert!(index < 64);
    ops::ldx(
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
pub unsafe fn load1024_y_aligned<T>(ptr: *const T, index: usize) {
    debug_assert!(index < 64);
    ops::ldy(
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
pub unsafe fn load1024_z_aligned<T>(ptr: *const T, index: usize) {
    debug_assert!(index < 64);
    ops::ldz(
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
pub unsafe fn store512_x<T>(ptr: *mut T, index: usize) {
    debug_assert!(index < 64);
    ops::stx(
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
pub unsafe fn store512_y<T>(ptr: *mut T, index: usize) {
    debug_assert!(index < 64);
    ops::sty(
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
pub unsafe fn store512_z<T>(ptr: *mut T, index: usize) {
    debug_assert!(index < 64);
    ops::stz(
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
pub unsafe fn store512_z_interleaved<T>(ptr: *mut T, index: usize) {
    debug_assert!(index < 64);
    ops::stzi(
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
pub unsafe fn store1024_x_aligned<T>(ptr: *mut T, index: usize) {
    debug_assert!(index < 64);
    ops::stx(
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
pub unsafe fn store1024_y_aligned<T>(ptr: *mut T, index: usize) {
    debug_assert!(index < 64);
    ops::sty(
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
pub unsafe fn store1024_z_aligned<T>(ptr: *mut T, index: usize) {
    debug_assert!(index < 64);
    ops::stz(
        MemArgs {
            ptr: ptr as *mut (),
            reg_offset: index as u64,
            size: MemSize::_128,
        }
        .encode(),
    );
}
