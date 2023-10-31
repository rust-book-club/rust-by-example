# Chapter 12: Cargo

I strongly encourage the use of `cargo` for Rust development.

It: builds your code, manages dependencies, publishes crates, runs tests, and much more.

## 12.1: Dependencies

Note, there are also

- [[dev-dependencies]](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies) (used for compiling tests, examples, and benchmarks, but not the main executable / library), and
- [[build-dependencies]](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#build-dependencies) (used for build scripts)

## 12.2: Conventions

This section explains how you might add multiple binaries to a single crate, but what if you wanted multiple crates in a single repo?

That's what [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) are for.

## 12.3: Tests

Put unit tests directly in the module.

Integration tests (in `/tests`) only have access to your public interface -- the same interface your clients have access to.

Many Rust devs use a TDD (Test Driven Development) style, where you

- write a test which tests some behaviour which does not yet exist
- implement _just enough_ behaviour to get the test to pass
- repeat

For instance, you might

- write a test which calls a method which does not yet exist
- implement the method
- write another test which passes a particular argument and expects a particular result
- directly return that value from the method
- write another test which passes some other arguments
- etc.

...updating existing tests as you go.

## 12.4: Build Scripts

Scripts that will run before your package is compiled.

You might not use them right away, but good to know that they exist.