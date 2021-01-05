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
pub fn enable() {
    unsafe { ops::set() };
}

/// Disable the coprocessor.
#[inline(always)]
pub unsafe fn disable() {
    ops::clr();
}

/// The parameters of AMX's load and store instructions.
#[derive(Debug, Copy, Clone)]
pub struct MemArgs {
    pub ptr: *mut (),
    /// 6-bit register offset (in units of `0x40`) in range `0..64`
    pub reg_offset: u8,
    pub size: MemSize,
}

impl MemArgs {
    #[inline]
    fn encode(self) -> u64 {
        debug_assert!(self.reg_offset < 64);

        (self.ptr as u64) & 0x00ff_ffff_ffff_ffff
            | ((self.reg_offset as u64) << 56)
            // [61] - ?
            | ((self.size as u64) << 62)
        // [63] - ?
    }
}

pub struct MemPayload(pub u64);

impl<T> From<(*const T, u8, MemSize)> for MemPayload {
    #[inline]
    fn from((ptr, reg_offset, size): (*const T, u8, MemSize)) -> MemPayload {
        MemArgs {
            ptr: ptr as _,
            reg_offset,
            size,
        }
        .into()
    }
}

impl<T> From<(*mut T, u8, MemSize)> for MemPayload {
    #[inline]
    fn from((ptr, reg_offset, size): (*mut T, u8, MemSize)) -> MemPayload {
        MemArgs {
            ptr: ptr as _,
            reg_offset,
            size,
        }
        .into()
    }
}

impl From<MemArgs> for MemPayload {
    #[inline]
    fn from(x: MemArgs) -> MemPayload {
        MemPayload(x.encode())
    }
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

#[inline(always)]
pub unsafe fn load_x(payload: impl Into<MemPayload>) {
    ops::ldx(payload.into().0);
}

#[inline(always)]
pub unsafe fn load_y(payload: impl Into<MemPayload>) {
    ops::ldy(payload.into().0);
}

#[inline(always)]
pub unsafe fn store_x(payload: impl Into<MemPayload>) {
    ops::stx(payload.into().0);
}

#[inline(always)]
pub unsafe fn store_y(payload: impl Into<MemPayload>) {
    ops::sty(payload.into().0);
}

#[inline(always)]
pub unsafe fn load_z(payload: impl Into<MemPayload>) {
    ops::ldz(payload.into().0);
}

#[inline(always)]
pub unsafe fn store_z(payload: impl Into<MemPayload>) {
    ops::stz(payload.into().0);
}

/// Interleaved load to `z` (the Z register).
///
/// [`MemArgs::size`] is ignored and assumed to be [`MemSize::_64`].
#[inline(always)]
pub unsafe fn load_z_interleaved(payload: impl Into<MemPayload>) {
    ops::ldzi(payload.into().0);
}

/// Interleaved store from `z` (the Z register).
///
/// [`MemArgs::size`] is ignored and assumed to be [`MemSize::_64`].
#[inline(always)]
pub unsafe fn store_z_interleaved(payload: impl Into<MemPayload>) {
    ops::stzi(payload.into().0);
}
