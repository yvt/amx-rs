//! AMX registers

/// Refers to a row (register) in the `x` register set.
///
/// The row index must be in range `0..8`.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct XRow(pub usize);

/// Refers to a row (register) in the `y` register set.
///
/// The row index must be in range `0..8`.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct YRow(pub usize);

/// Refers to a row (register) in the `z` register set.
///
/// The row index must be in range `0..64`.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ZRow(pub usize);

/// A byte offset in `x` register set.
///
/// The byte offset must be in range `0..512`.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct XBytes(pub usize);

/// A byte offset in `y` register set.
///
/// The byte offset must be in range `0..512`.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct YBytes(pub usize);
