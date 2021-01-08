# `amx`

[<img src="https://docs.rs/amx/badge.svg" alt="docs.rs">](https://docs.rs/amx/)

Rust wrapper for Apple Matrix Coprocessor (AMX) instructions

This crate provides wrapper functions for the undocumented AMX instructions,
which are found in Apple Silicon processors.

## Resources

 - <https://gist.github.com/dougallj/7a75a3be1ec69ca550e7c36dc75e0d6f>
 - <https://www.realworldtech.com/forum/?threadid=187087&curpostid=187120>

## Example

```rust
unsafe {
    let x = [1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 11, 12, 13, 14, 15, 16,
             17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32i16];
    let y = [51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66,
             67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82i16];
    amx::enable();
    amx::load512_x(x.as_ptr(), 0);
    amx::load512_y(y.as_ptr(), 0);
    amx::outer_product_i16_xy_to_z(
        0,     // input from X starting from byte offset 0
        0,     // input from Y starting from byte offset 0
        0,     // output to Z starting from row offset 0
        false, // don't accumulate
        false, // use X
        false, // use Y
    );
    let z: [[i16; 32]; 64] = std::mem::transmute(amx::read_z());
    for (x_i, &x) in x.iter().enumerate() {
        for (y_i, &y) in y.iter().enumerate() {
            assert_eq!(z[y_i * 2][x_i], x * y);
        }
    }
    amx::disable();
}
```

## Registers

```rust
struct AmxState {
    /// "8 64-byte registers"
    x: [[u8; 64]; 8],
    /// "8 64-byte registers"
    y: [[u8; 64]; 8],
    /// "64 64-byte registers in an M-by-N matrix"
    z: [[u8; 64]; 64],
}
```
