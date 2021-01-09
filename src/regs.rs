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
