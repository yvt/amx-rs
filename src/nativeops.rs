//! Low-level operations (modeled after [Apple compiler intrinsics])
//!
//! [Apple compiler intrinsics]: https://www.realworldtech.com/forum/?threadid=187087&curpostid=187120
use std::{arch::asm, marker::PhantomData};

/// Emit an AMX instruction with an input register.
#[inline(always)]
pub unsafe fn op_in<const OP: u8>(operand: u64) {
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

/// Emit an AMX instruction with a 5-bit immediate.
#[inline(always)]
pub unsafe fn op_imm<const OP: u8, const OPERAND: u8>() {
    asm!(
        ".word 0x00201000 + ({op} << 5) + {operand}",
        op = const OP,
        operand = const OPERAND,
        options(nostack, preserves_flags),
    );
}

#[inline(always)]
pub unsafe fn ldx(x: u64) {
    op_in::<0>(x);
}

#[inline(always)]
pub unsafe fn ldy(x: u64) {
    op_in::<1>(x);
}

#[inline(always)]
pub unsafe fn stx(x: u64) {
    op_in::<2>(x);
}

#[inline(always)]
pub unsafe fn sty(x: u64) {
    op_in::<3>(x);
}

#[inline(always)]
pub unsafe fn ldz(x: u64) {
    op_in::<4>(x);
}

#[inline(always)]
pub unsafe fn stz(x: u64) {
    op_in::<5>(x);
}

#[inline(always)]
pub unsafe fn ldzi(x: u64) {
    op_in::<6>(x);
}

#[inline(always)]
pub unsafe fn stzi(x: u64) {
    op_in::<7>(x);
}

#[inline(always)]
pub unsafe fn extrx(x: u64) {
    op_in::<8>(x);
}

#[inline(always)]
pub unsafe fn extry(x: u64) {
    op_in::<9>(x);
}

#[inline(always)]
pub unsafe fn fma64(x: u64) {
    op_in::<10>(x);
}

#[inline(always)]
pub unsafe fn fms64(x: u64) {
    op_in::<11>(x);
}

#[inline(always)]
pub unsafe fn fma32(x: u64) {
    op_in::<12>(x);
}

#[inline(always)]
pub unsafe fn fms32(x: u64) {
    op_in::<13>(x);
}

#[inline(always)]
pub unsafe fn mac16(x: u64) {
    op_in::<14>(x);
}

#[inline(always)]
pub unsafe fn fma16(x: u64) {
    op_in::<15>(x);
}

#[inline(always)]
pub unsafe fn fms16(x: u64) {
    op_in::<16>(x);
}

#[inline(always)]
pub unsafe fn set() {
    op_imm::<17, 0>();
}

#[inline(always)]
pub unsafe fn clr() {
    op_imm::<17, 1>();
}

#[inline(always)]
pub unsafe fn vecint(x: u64) {
    op_in::<18>(x);
}

#[inline(always)]
pub unsafe fn vecfp(x: u64) {
    op_in::<19>(x);
}

#[inline(always)]
pub unsafe fn matint(x: u64) {
    op_in::<20>(x);
}

#[inline(always)]
pub unsafe fn matfp(x: u64) {
    op_in::<21>(x);
}

#[inline(always)]
pub unsafe fn genlut(x: u64) {
    op_in::<22>(x);
}

/// Exposes the target processor's AMX support by implementing [`AmxOps`] trait.
///
/// [`AmxOps`]: crate::ops::AmxOps
///
/// The lifetime parameter can be used to restrict the use of `&AmxOps` to
/// a scope where AMX is enabled. To make this technique effective, this type
/// does not implement `Clone`. (GAT would make this more effective.)
///
/// This type is not `Send`-able because AMX states, including whether it's
/// enabled, are thread-local.
pub struct AmxOps<'a>(PhantomData<(&'a mut (), *mut ())>);

impl<'a> AmxOps<'a> {
    /// Construct `Self`.
    ///
    /// # Safety
    ///
    /// The following conditions must be satisfied on any use of this type's
    /// `AmxOps` methods:
    ///
    ///  - The target processor must actually support AMX.
    ///
    /// (Calling the methods while AMX being disabled is no unsafer than calling
    /// `abort`, I think.)
    #[inline]
    pub unsafe fn new() -> Self {
        Self(PhantomData)
    }

    /// Reborrow `self`, constructing a new `AmxOps` with a narrower lifetime.
    #[inline]
    pub fn borrow_mut(&mut self) -> AmxOps<'_> {
        Self(PhantomData)
    }
}

unsafe impl crate::ops::AmxOps for AmxOps<'_> {
    #[inline(always)]
    unsafe fn ldx(&mut self, x: u64, ptr: *mut ()) {
        ldx(x | (ptr as u64 & 0x00ff_ffff_ffff_ffff));
    }
    #[inline(always)]
    unsafe fn ldy(&mut self, x: u64, ptr: *mut ()) {
        ldy(x | (ptr as u64 & 0x00ff_ffff_ffff_ffff));
    }
    #[inline(always)]
    unsafe fn stx(&mut self, x: u64, ptr: *mut ()) {
        stx(x | (ptr as u64 & 0x00ff_ffff_ffff_ffff));
    }
    #[inline(always)]
    unsafe fn sty(&mut self, x: u64, ptr: *mut ()) {
        sty(x | (ptr as u64 & 0x00ff_ffff_ffff_ffff));
    }
    #[inline(always)]
    unsafe fn ldz(&mut self, x: u64, ptr: *mut ()) {
        ldz(x | (ptr as u64 & 0x00ff_ffff_ffff_ffff));
    }
    #[inline(always)]
    unsafe fn stz(&mut self, x: u64, ptr: *mut ()) {
        stz(x | (ptr as u64 & 0x00ff_ffff_ffff_ffff));
    }
    #[inline(always)]
    unsafe fn ldzi(&mut self, x: u64, ptr: *mut ()) {
        ldzi(x | (ptr as u64 & 0x00ff_ffff_ffff_ffff));
    }
    #[inline(always)]
    unsafe fn stzi(&mut self, x: u64, ptr: *mut ()) {
        stzi(x | (ptr as u64 & 0x00ff_ffff_ffff_ffff));
    }
    #[inline(always)]
    fn extrx(&mut self, x: u64) {
        unsafe { extrx(x) };
    }
    #[inline(always)]
    fn extry(&mut self, x: u64) {
        unsafe { extry(x) };
    }
    #[inline(always)]
    fn fma64(&mut self, x: u64) {
        unsafe { fma64(x) };
    }
    #[inline(always)]
    fn fms64(&mut self, x: u64) {
        unsafe { fms64(x) };
    }
    #[inline(always)]
    fn fma32(&mut self, x: u64) {
        unsafe { fma32(x) };
    }
    #[inline(always)]
    fn fms32(&mut self, x: u64) {
        unsafe { fms32(x) };
    }
    #[inline(always)]
    fn mac16(&mut self, x: u64) {
        unsafe { mac16(x) };
    }
    #[inline(always)]
    fn fma16(&mut self, x: u64) {
        unsafe { fma16(x) };
    }
    #[inline(always)]
    fn fms16(&mut self, x: u64) {
        unsafe { fms16(x) };
    }
    #[inline(always)]
    fn vecint(&mut self, x: u64) {
        unsafe { vecint(x) };
    }
    #[inline(always)]
    fn vecfp(&mut self, x: u64) {
        unsafe { vecfp(x) };
    }
    #[inline(always)]
    fn matint(&mut self, x: u64) {
        unsafe { matint(x) };
    }
    #[inline(always)]
    fn matfp(&mut self, x: u64) {
        unsafe { matfp(x) };
    }
    #[inline(always)]
    fn genlut(&mut self, x: u64) {
        unsafe { genlut(x) };
    }
}
