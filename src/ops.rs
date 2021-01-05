//! Low-level operations (modeled after [Apple compiler intrinsics])
//!
//! [Apple compiler intrinsics]: https://www.realworldtech.com/forum/?threadid=187087&curpostid=187120

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
