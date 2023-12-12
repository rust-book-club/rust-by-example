# Chapter 21: Testing

## 21.1 Unit Testing

In Rust, unit tests live in the same file as the library code they are testing.

Minimal example

```rs
// lib/add.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)] // tests are compiled only in test scope
mod tests {
    use super::*; // import code (which we want to test) from the outer scope

    #[test] // only fns marked with #[test] are treated as tests
    fn test_add() { // convention: begin test fn names with "test_"
        assert_eq!(add(1, 2), 3); // assert!(), assert_ne!() also available
    }
}
```

Run tests with `cargo test`.

### Tests and ?

Tests can explicity return a `Result`, and then `?` can be used within them.

See example in book.

### Testing panics

Use the `#[should_panic]` attribute to test code that is _supposed_ to panic.

### Running specific tests

Type anything after `cargo test` to filter to tests with that thing in the name of the test

See example in book.

### Ignoring tests

Use the `#[ignore]` attribute on a `#[test]` to put it into a special category of "ignored tests".

Ignored tests will not run on a `cargo test`.

`cargo test --ignored` will run _only_ ignored tests.

## 21.2 Documentation Testing

Code blocks written in documentation comments `///` are run as _documentation tests_. `cargo test` runs these tests.

### Motivation behind documentation tests

You can hide "setup code" required for doctests. The code will be run, but not displayed to the user.

## 21.3 Integration testing

> "Integration tests are external to your crate and use only its public interface in the same way any other code would."

Integration tests go in a `tests/` directory.

## 21.4 Dev-dependencies

The `[dev-dependencies]` section in your `Cargo.toml` file defines dependencies which are needed for tests only.