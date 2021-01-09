//! AMX emulation
use crate::ops::AmxOps;

/// An emulated AMX context.
#[derive(Default, Debug, Copy, Clone)]
pub struct AmxEmuCtx {
    st: AmxSt,
}

impl AmxEmuCtx {
    /// Construct a brand new `AmxEmuCtx` initialized with a default state.
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Copy, Clone)]
struct AmxSt {
    /// "8 64-byte registers"
    x: [u8; 512],
    /// "8 64-byte registers"
    y: [u8; 512],
    /// "64 64-byte registers in an M-by-N matrix"
    z: [u8; 4096],
}

// FIXME: Large arrays do not implement `Default` yet (This is complicated
//        because the current `Default` impl for `[T; 0]` doesn't have a `T:
//        Default` bound, so simply replacing it with const generics would be a
//        breaking change:
//        <https://github.com/rust-lang/rust/pull/60466#discussion_r280989938>)
impl Default for AmxSt {
    fn default() -> Self {
        Self {
            x: [0; 512],
            y: [0; 512],
            z: [0; 4096],
        }
    }
}

unsafe impl AmxOps for AmxEmuCtx {
    unsafe fn ldx(&mut self, x: u64, ptr: *mut ()) {
        todo!()
    }

    unsafe fn ldy(&mut self, x: u64, ptr: *mut ()) {
        todo!()
    }

    unsafe fn stx(&mut self, x: u64, ptr: *mut ()) {
        todo!()
    }

    unsafe fn sty(&mut self, x: u64, ptr: *mut ()) {
        todo!()
    }

    unsafe fn ldz(&mut self, x: u64, ptr: *mut ()) {
        todo!()
    }

    unsafe fn stz(&mut self, x: u64, ptr: *mut ()) {
        todo!()
    }

    unsafe fn ldzi(&mut self, x: u64, ptr: *mut ()) {
        todo!()
    }

    unsafe fn stzi(&mut self, x: u64, ptr: *mut ()) {
        todo!()
    }

    fn extrx(&mut self, x: u64) {
        todo!()
    }

    fn extry(&mut self, x: u64) {
        todo!()
    }

    fn fma64(&mut self, x: u64) {
        todo!()
    }

    fn fms64(&mut self, x: u64) {
        todo!()
    }

    fn fma32(&mut self, x: u64) {
        todo!()
    }

    fn fms32(&mut self, x: u64) {
        todo!()
    }

    fn mac16(&mut self, x: u64) {
        todo!()
    }

    fn fma16(&mut self, x: u64) {
        todo!()
    }

    fn fms16(&mut self, x: u64) {
        todo!()
    }

    fn vecint(&mut self, x: u64) {
        todo!()
    }

    fn vecfp(&mut self, x: u64) {
        todo!()
    }

    fn matint(&mut self, x: u64) {
        todo!()
    }

    fn matfp(&mut self, x: u64) {
        todo!()
    }

    fn genlut(&mut self, x: u64) {
        todo!()
    }
}
