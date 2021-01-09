/// Exposes all AMX instructions except `set` and `clr` as trait methods.
///
/// Load and store operations receive a pointer by the additional parameter to
/// allow emulation on a system with a different pointer size.
pub unsafe trait AmxOps {
    unsafe fn ldx(&mut self, x: u64, ptr: *mut ());
    unsafe fn ldy(&mut self, x: u64, ptr: *mut ());
    unsafe fn stx(&mut self, x: u64, ptr: *mut ());
    unsafe fn sty(&mut self, x: u64, ptr: *mut ());
    unsafe fn ldz(&mut self, x: u64, ptr: *mut ());
    unsafe fn stz(&mut self, x: u64, ptr: *mut ());
    unsafe fn ldzi(&mut self, x: u64, ptr: *mut ());
    unsafe fn stzi(&mut self, x: u64, ptr: *mut ());
    fn extrx(&mut self, x: u64);
    fn extry(&mut self, x: u64);
    fn fma64(&mut self, x: u64);
    fn fms64(&mut self, x: u64);
    fn fma32(&mut self, x: u64);
    fn fms32(&mut self, x: u64);
    fn mac16(&mut self, x: u64);
    fn fma16(&mut self, x: u64);
    fn fms16(&mut self, x: u64);
    fn vecint(&mut self, x: u64);
    fn vecfp(&mut self, x: u64);
    fn matint(&mut self, x: u64);
    fn matfp(&mut self, x: u64);
    fn genlut(&mut self, x: u64);
}

// Safety: Just forwarding the calls
unsafe impl<T: ?Sized + AmxOps> AmxOps for &'_ mut T {
    unsafe fn ldx(&mut self, x: u64, ptr: *mut ()) {
        (**self).ldx(x, ptr)
    }
    unsafe fn ldy(&mut self, x: u64, ptr: *mut ()) {
        (**self).ldy(x, ptr)
    }
    unsafe fn stx(&mut self, x: u64, ptr: *mut ()) {
        (**self).stx(x, ptr)
    }
    unsafe fn sty(&mut self, x: u64, ptr: *mut ()) {
        (**self).sty(x, ptr)
    }
    unsafe fn ldz(&mut self, x: u64, ptr: *mut ()) {
        (**self).ldz(x, ptr)
    }
    unsafe fn stz(&mut self, x: u64, ptr: *mut ()) {
        (**self).stz(x, ptr)
    }
    unsafe fn ldzi(&mut self, x: u64, ptr: *mut ()) {
        (**self).ldzi(x, ptr)
    }
    unsafe fn stzi(&mut self, x: u64, ptr: *mut ()) {
        (**self).stzi(x, ptr)
    }
    fn extrx(&mut self, x: u64) {
        (**self).extrx(x)
    }
    fn extry(&mut self, x: u64) {
        (**self).extry(x)
    }
    fn fma64(&mut self, x: u64) {
        (**self).fma64(x)
    }
    fn fms64(&mut self, x: u64) {
        (**self).fms64(x)
    }
    fn fma32(&mut self, x: u64) {
        (**self).fma32(x)
    }
    fn fms32(&mut self, x: u64) {
        (**self).fms32(x)
    }
    fn mac16(&mut self, x: u64) {
        (**self).mac16(x)
    }
    fn fma16(&mut self, x: u64) {
        (**self).fma16(x)
    }
    fn fms16(&mut self, x: u64) {
        (**self).fms16(x)
    }
    fn vecint(&mut self, x: u64) {
        (**self).vecint(x)
    }
    fn vecfp(&mut self, x: u64) {
        (**self).vecfp(x)
    }
    fn matint(&mut self, x: u64) {
        (**self).matint(x)
    }
    fn matfp(&mut self, x: u64) {
        (**self).matfp(x)
    }
    fn genlut(&mut self, x: u64) {
        (**self).genlut(x)
    }
}
