/// Exposes all AMX instructions except `set` and `clr` as trait methods.
pub unsafe trait AmxOps {
    unsafe fn ldx(&mut self, x: u64);
    unsafe fn ldy(&mut self, x: u64);
    unsafe fn stx(&mut self, x: u64);
    unsafe fn sty(&mut self, x: u64);
    unsafe fn ldz(&mut self, x: u64);
    unsafe fn stz(&mut self, x: u64);
    unsafe fn ldzi(&mut self, x: u64);
    unsafe fn stzi(&mut self, x: u64);
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
