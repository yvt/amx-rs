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
