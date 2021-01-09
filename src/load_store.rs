use crate::{
    regs::{XRow, YRow, ZRow},
    AmxOps,
};

/// The parameters of AMX's load and store instructions.
#[derive(Copy, Clone)]
struct MemArgs {
    /// 6-bit register offset (in units of `0x40`) in range `0..64`
    reg_offset: u64,
    size: MemSize,
}

impl MemArgs {
    #[inline]
    fn encode(self) -> u64 {
        debug_assert!(self.reg_offset < 64);

        // The pointer is passed by a separate parameter when using `AmxOps`
        (self.reg_offset << 56)
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

/// Register row types supporting 512-bit and 1024-bit operations.
///
/// This trait is not meant to be used directly. Please use [`Amx`]'s methods
/// instead.
///
/// [`Amx`]: crate::Amx
pub trait LoadStore {
    /// Load 512 bits (64 bytes) from memory to the register.
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T);
    /// Store 512 bits (64 bytes) to memory from the register.
    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T);

    /// Load 1024 bits (128 bytes) from memory to the register.
    ///
    /// `ptr` must be aligned to 128-byte boundaries.
    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T);
    /// Store 1024 bits (128 bytes) to memory from the register.
    ///
    /// `ptr` must be aligned to 128-byte boundaries.
    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T);
}

#[cfg(feature = "either")]
impl<Left: LoadStore, Right: LoadStore> LoadStore for either::Either<Left, Right> {
    #[inline]
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        match self {
            either::Left(x) => x.load512(ops, ptr),
            either::Right(x) => x.load512(ops, ptr),
        }
    }

    #[inline]
    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        match self {
            either::Left(x) => x.store512(ops, ptr),
            either::Right(x) => x.store512(ops, ptr),
        }
    }

    #[inline]
    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        match self {
            either::Left(x) => x.load1024_aligned(ops, ptr),
            either::Right(x) => x.load1024_aligned(ops, ptr),
        }
    }

    #[inline]
    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        match self {
            either::Left(x) => x.store1024_aligned(ops, ptr),
            either::Right(x) => x.store1024_aligned(ops, ptr),
        }
    }
}

impl LoadStore for XRow {
    #[inline(always)]
    #[track_caller]
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        assert!(index < 8);
        ops.ldx(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        assert!(index < 8);
        ops.stx(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        assert!(index < 8);
        ops.ldx(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        assert!(index < 8);
        ops.stx(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }
}

impl LoadStore for YRow {
    #[inline(always)]
    #[track_caller]
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        assert!(index < 8);
        ops.ldy(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        assert!(index < 8);
        ops.sty(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        assert!(index < 8);
        ops.ldy(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        assert!(index < 8);
        ops.sty(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }
}

impl LoadStore for ZRow {
    #[inline(always)]
    #[track_caller]
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        assert!(index < 64);
        ops.ldz(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        assert!(index < 64);
        ops.stz(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        assert!(index < 64);
        ops.ldz(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        assert!(index < 64);
        ops.stz(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }
}

/// Load 512 bits (64 bytes) from memory to `z[index][0..64]` with interleaving.
///
/// `index` must be in range `0..64`.
#[inline(always)]
#[track_caller]
pub(crate) unsafe fn load512_z_interleaved<T>(
    ops: &mut (impl AmxOps + ?Sized),
    ptr: *const T,
    ZRow(index): ZRow,
) {
    assert!(index < 64);
    ops.ldzi(
        MemArgs {
            reg_offset: index as u64,
            size: MemSize::_64,
        }
        .encode(),
        ptr as *mut (),
    );
}

/// Store 512 bits (64 bytes) `z[index][0..64]` to memory with interleaving.
///
/// `index` must be in range `0..64`.
#[inline(always)]
#[track_caller]
pub(crate) unsafe fn store512_z_interleaved<T>(
    ops: &mut (impl AmxOps + ?Sized),
    ptr: *mut T,
    ZRow(index): ZRow,
) {
    assert!(index < 64);
    ops.stzi(
        MemArgs {
            reg_offset: index as u64,
            size: MemSize::_64,
        }
        .encode(),
        ptr as *mut (),
    );
}
