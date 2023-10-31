# Chapter 11: Crates

[Like `#include` in C++](https://stackoverflow.com/a/35720784), `mod` in Rust literally includes the contents of that other file at that point in the code.

> "...`use` simply brings an item into the current namespace so you can access it more easily. Whereas `mod` (without a body block `{}`) literally brings the contents of a file and inserts in its place." [[source]](https://panaetius.io/post/2020/11/the-difference-between-mod-and-use-in-rust/)

The _crate_ is the smallest compilation unit in Rust. Modules do not get compiled individually.

## 11.1: Creating a Library

Libraries can be brought into other Rust projects as crates; binaries cannot, they can only be executed.

## 11.2: Using a Library

This section concerns compiling code locally using `rustc`. Usually, we let `cargo` do the heavy lifting and linking here.