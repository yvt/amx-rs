//! Wrapper for the `genlut` instruction
use crate::{
    regs::{XBytes, XRow, YBytes, YRow, ZRow},
    AmxOps,
};

/// The trait for marker types specifying `genlut` instruction's LUT type.
///
/// The types implementing this trait are in the following form:
/// `(direction, index, value)`.
///
///  - `direction` is either [`Normal`] or [`Reverse`].
///  - `index` is the data type for indices.
///  - `value` is the data type for looked-up values.
///
pub trait LutTy {
    /// The raw LUT mode number for `genlut` instruction.
    fn genlut_mode(&self) -> u64;
}

/// Specifies the normal application of a look-up table.
pub struct Normal;
/// Specifies the reversed application of a look-up table.
pub struct Reverse;

/// Two-bit LUT indices.
pub struct Index2;

/// Four-bit LUT indices.
pub struct Index4;

/// Five-bit (tightly-packed) LUT indices.
pub struct Index5;

/// A tag for the 16-bit floating-point number data type.
pub struct F16;

/// A tag for the 32-bit floating-point number data type.
pub struct F32;

/// A tag for the 64-bit floating-point number data type.
pub struct F64;

/// A tag for the 16-bit unsigned integer type.
pub struct U16;

/// A tag for the 32-bit unsigned integer type.
pub struct U32;

/// A tag for the 64-bit unsigned integer type.
pub struct U64;

/// A tag for the 16-bit signed integer type.
pub struct I16;

/// A tag for the 32-bit signed integer type.
pub struct I32;

/// A tag for the 64-bit signed integer type.
pub struct I64;

/// A tag for an 8-bit type.
pub struct X8;

/// A tag for a 16-bit type.
pub struct X16;

/// A tag for a 32-bit type.
pub struct X32;

/// A tag for a 64-bit type.
pub struct X64;

macro_rules! define_lut_ty {
    ($(
        $ty:ty => $val:expr
    ),*$(,)*) => {$(
        impl LutTy for $ty {
            #[inline(always)]
            fn genlut_mode(&self) -> u64 {
                $val
            }
        }
    )*};
}

define_lut_ty! {
    (Reverse, Index4, F32) => 0,
    (Reverse, Index5, F16) => 1,
    (Reverse, Index4, F64) => 2,
    (Reverse, Index4, I32) => 3,
    (Reverse, Index5, I16) => 4,
    (Reverse, Index4, U32) => 5,
    (Reverse, Index5, U16) => 6,
    (Normal, Index2, X32) => 7,
    (Normal, Index2, X16) => 8,
    (Normal, Index2, X8) => 9,
    (Normal, Index4, X64) => 10,
    (Normal, Index4, X32) => 11,
    (Normal, Index4, X16) => 12,
    (Normal, Index4, X8) => 13,
    (Normal, Index5, X16) => 14,
    (Normal, Index5, X8) => 15,
}

#[cfg(feature = "either")]
impl<Left: LutTy, Right: LutTy> LutTy for either::Either<Left, Right> {
    #[inline]
    fn genlut_mode(&self) -> u64 {
        match self {
            either::Left(x) => x.genlut_mode(),
            either::Right(x) => x.genlut_mode(),
        }
    }
}

/// The trait representing `genlut` instruction's input, which can be either
/// [`XBytes`] or [`YBytes`].
pub trait LutIn {
    fn as_genlut_input_param(&self) -> u64;
}

impl LutIn for XBytes {
    #[inline(always)]
    fn as_genlut_input_param(&self) -> u64 {
        debug_assert!(self.0 < 512);
        self.0 as u64
    }
}

impl LutIn for YBytes {
    #[inline(always)]
    fn as_genlut_input_param(&self) -> u64 {
        debug_assert!(self.0 < 512);
        self.0 as u64 | (1u64 << 10) // "input is in Y"
    }
}

#[cfg(feature = "either")]
impl<Left: LutIn, Right: LutIn> LutIn for either::Either<Left, Right> {
    #[inline]
    fn as_genlut_input_param(&self) -> u64 {
        match self {
            either::Left(x) => x.as_genlut_input_param(),
            either::Right(x) => x.as_genlut_input_param(),
        }
    }
}

/// The trait representing `genlut` instruction's output, which can be either
/// [`XRow`], [`YRow`], or [`ZRow`].
pub trait LutOut {
    fn as_genlut_output_param(&self) -> u64;
}

impl LutOut for XRow {
    #[inline(always)]
    fn as_genlut_output_param(&self) -> u64 {
        debug_assert!(self.0 < 8);
        (self.0 as u64) << 20
    }
}

impl LutOut for YRow {
    #[inline(always)]
    fn as_genlut_output_param(&self) -> u64 {
        debug_assert!(self.0 < 8);
        ((self.0 as u64) << 20) | (1u64 << 25) // "input is in Y"
    }
}

impl LutOut for ZRow {
    #[inline(always)]
    fn as_genlut_output_param(&self) -> u64 {
        debug_assert!(self.0 < 64);
        ((self.0 as u64) << 20) | (1u64 << 26)
    }
}

#[cfg(feature = "either")]
impl<Left: LutOut, Right: LutOut> LutOut for either::Either<Left, Right> {
    #[inline]
    fn as_genlut_output_param(&self) -> u64 {
        match self {
            either::Left(x) => x.as_genlut_output_param(),
            either::Right(x) => x.as_genlut_output_param(),
        }
    }
}

#[inline(always)]
pub(crate) fn lut(
    ops: &mut (impl AmxOps + ?Sized),
    input: impl LutIn,
    XRow(table_row): XRow,
    output: impl LutOut,
    mode: impl LutTy,
) {
    ops.genlut(
        input.as_genlut_input_param()
            | output.as_genlut_output_param()
            | (mode.genlut_mode() << 53)
            | ((table_row as u64) << 60),
    );
}
