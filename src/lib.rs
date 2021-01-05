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
//!     /// "8 64-byte registers", a.k.a. `x`
//!     amx0: [[u8; 64]; 8],
//!     /// "8 64-byte registers", a.k.a. `y`
//!     amx1: [[u8; 64]; 8],
//!     /// "64 64-byte registers in an M-by-N matrix", a.k.a. `z`
//!     amx2: [[u8; 64]; 64],
//! }
//! ```
#![feature(asm)]

#[inline(always)]
unsafe fn op_reg<const OP: u8>(operand: u64) {
    asm!(
        // Most AMX instructions take a 64-bit register number (e.g., `x25`) as
        // the operand. The problem is how to encode the register number in the
        // `.word` directive. We could use a fixed register number, but then we
        // would have to move a value into that register, which is utterly
        // inefficient.
        //
        // The trick used here is to prepend `0` to the register name to make it
        // look like a hexadecimal number (e.g., `0x25`). The encoding is still
        // wrong at this point because the register number is a decimal number,
        // but it's being interpreted as a hexadecimal number (`0x25 = 37`). So
        // we have to split it into digits (`2` and `5`) and reconstitute the
        // intended register number (`2 * 10 + 5`).
        ".word 0x00201000 + ({op} << 5) + (0{operand} & 0xf) + (0{operand} >> 4) * 10",
        op = const OP,
        operand = in(reg) operand,
        options(nostack, preserves_flags),
    );
}

#[inline(always)]
unsafe fn op_imm<const OP: u8, const OPERAND: u8>() {
    asm!(
        ".word 0x00201000 + ({op} << 5) + {operand}",
        op = const OP,
        operand = const OPERAND,
        options(nostack, preserves_flags),
    );
}

#[inline(always)]
pub fn enable() {
    unsafe { op_imm::<17, 0>() };
}

#[inline(always)]
pub fn disable() {
    unsafe { op_imm::<17, 1>() };
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
pub unsafe fn load_amx0(payload: impl Into<MemPayload>) {
    op_reg::<0>(payload.into().0);
}

#[inline(always)]
pub unsafe fn load_amx1(payload: impl Into<MemPayload>) {
    op_reg::<1>(payload.into().0);
}

#[inline(always)]
pub unsafe fn store_amx0(payload: impl Into<MemPayload>) {
    op_reg::<2>(payload.into().0);
}

#[inline(always)]
pub unsafe fn store_amx1(payload: impl Into<MemPayload>) {
    op_reg::<3>(payload.into().0);
}

#[inline(always)]
pub unsafe fn load_amx2(payload: impl Into<MemPayload>) {
    op_reg::<4>(payload.into().0);
}

#[inline(always)]
pub unsafe fn store_amx2(payload: impl Into<MemPayload>) {
    op_reg::<5>(payload.into().0);
}

/// Interleaved load to `amx2` (the Z register).
///
/// [`MemArgs::size`] is ignored and assumed to be [`MemSize::_64`].
#[inline(always)]
pub unsafe fn load_amx2_interleaved(payload: impl Into<MemPayload>) {
    op_reg::<6>(payload.into().0);
}

/// Interleaved store from `amx2` (the Z register).
///
/// [`MemArgs::size`] is ignored and assumed to be [`MemSize::_64`].
#[inline(always)]
pub unsafe fn store_amx2_interleaved(payload: impl Into<MemPayload>) {
    op_reg::<7>(payload.into().0);
}
