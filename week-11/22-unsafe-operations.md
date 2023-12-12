# Chapter 22: Unsafe Operations

`unsafe` Rust code does _not_ let you do anything you want.

It lets you do [five](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#unsafe-superpowers) specific things

1. Dereference a raw pointer
2. Call an `unsafe` function or method
3. Access or modify a mutable `static` variable
4. Implement an `unsafe` trait
5. Access fields of `union`s (_Rust by Example_ does not mention this one)

You will mostly use `unsafe` blocks when interfacing with C code (points 1 and 5), but it's also required in some pure Rust code (points 2 and 4) when implementing self-referential data types like graphs, for example.

## 22.1 Inline assembly

Rust lets you directly write assembly code for several architectures (x86, x86-64, ARM, AArch64, RISC-V).

```rs
// example
use std::arch::asm;

let x: u64;
unsafe {
    asm!("mov {}, 5", out(reg) x);
}
assert_eq!(x, 5);
```

Lots of details in this section specific to writing assembly in Rust... if that's your thing.