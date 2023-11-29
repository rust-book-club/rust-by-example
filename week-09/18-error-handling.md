# Chapter 18: Error handling

`panic` vs `Option` vs `Result` -- when to use?

Similar to Scala:
- `panic` is an explicit failure, like throwing an `Exception` or calling `System.exit`
- `Option` is like `Option` in Scala
    - "when the lack of a value is not an error condition"
- `Result` is like `Try` in Scala
    - `unwrap` and `expect` are like `get` in Scala

## 18.1 panic

`panic` is like throwing an `Exception` in Java / Scala.

`Exception`s are easy to catch and handle in the JVM, but you generally do not catch `panic`s ([although it is possible](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) in some cases). It is not recommended to do this.

## 18.2 abort & unwind

An `unwind` panic is the kind that can be caught with `catch_unwind`. `abort` panics cannot be caught, they immediately abort the process.

As a program author, why would you want to `unwind` vs. `abort`? `abort`ing [makes the resulting binary a bit smaller](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic), and leaves it to the operating system to clean up the memory used by the program.

> With the new feature merged in, one can opt out formatting and panic handling using `panic_immediate_abort` and `#![no_main]`. This can bring static executable size under 20 kilobytes. [source](https://github.com/rust-lang/rust/issues/54981#issuecomment-443369450)

## 18.3 Option & unwrap

Rust's `Option` is very similar to Scala's `Option`.

`expect` and `unwrap` are basically identical, except `expect` [allows you to include a custom error message](https://stackoverflow.com/a/61302112) when the `Option` is `None`.

### 18.3.1 Unpacking options with ?

`?` on an `Option` is something like

```
match my_option {
    None        => return None,  // if None, abort this function / method immediately
    Some(value) => value         // if Some, unwrap
}
```

### 18.3.2 Combinators: map

Create your own `Option<T> -> Option<T>` methods and compose them into [combinators](https://softwareengineering.stackexchange.com/a/117575).

### 18.3.3 Combinators: and_then

`and_then` is like `flatMap` in Scala.

### 18.3.4 Unpacking options and defaults

- "keeps empty value intact" means we can use `let`.
- "modifies empty value in place" means we have to use `let mut`.

This phrasing is needlessly confusing, in my opinion.

Note that "keeps empty value intact" does _not_ mean that we can use the empty value later. Adding

```rs
println!("{:?}", no_fruit)
```

to the end of either of the first two examples results in a compiler error (use after move).

## 18.4 Result

`Result` is like `Try` in Scala.

`Result` differs from `Option` semantically in that a `None` is not necessarily a _failure_. As explained earlier in the book, the root directory `/` (or `C:\\` on Windows) has no parent directory. Representing that as a `None` is fine, and is not an error.

`Result::Err`, however, does imply that something unusual or unexpected happened. `Result::Ok` implies that we are on the "happy path" -- that some fallible process has succeeded (like reading a file, or opening a network connection, or getting user input).

### Using Result in main

`main()` [returns `ExitCode::SUCCESS` by default](https://doc.rust-lang.org/std/process/trait.Termination.html#impl-Termination-for-()).

But `main()` can also return a `Result`, to give a slightly nicer message when there's an error.

### 18.4.1 map for Result

Like with `Try` in Scala, `map` and `and_then` (`flatMap`) can be applied to `Result`s in Rust, as well.

`Ok`s will be mapped using the function, and `Err`s will pass through as-is.

### 18.4.2 aliases for Result

`std::io::Result` is an alias for `std::result::Result<T, std::io::Error>;`

It's good to know that this exists, because at some point you will probably confuse `io::Result` with `std::result::Result`.

Be aware that you can create result aliases, but also that they might be confusing and unnecessary.

### 18.4.3 Early returns

Early returns are frowned upon in Scala (and FP in general), but they can be used in Rust to simplify your code. In this case, we're reducing the number of nested `match` statements which we might otherwise need.

### 18.4.4 Introducing ?

Use `?` on a `Result` to return `Err` instead of `panic`king.

Similar to using `?` on an `Option`.

In old code, you may see `try!(<expr>)` instead of `<expr>?`. It is equivalent.

Rust does this sometimes: incorporates macros into the language. See: [lazy_static!](https://docs.rs/lazy_static/latest/lazy_static/) vs. [OnceCell](https://docs.rs/once_cell/latest/once_cell/#lazy-initialized-global-data), as discussed on Teams.

### 18.5 Multiple error types

How do we work with different kinds of monads (`Option` and `Result` in the same block)?

You may have encountered this problem in Scala, as well, when using `for` comprehensions to map `Future`s and `Option`s at the same time.

There is a surprising amount of complexity around handling errors of different types in Rust, as we will see.

### 18.5.1 Pulling Results out of Options

Run these examples...

`Result`s wrapped in `Option`s (and vice versa) are kind of clunky.

Is there a better way?

### 18.5.2 Defining an error type

Convert "built-in" errors to our own custom error type.

Note there are crates to help you define error types, like [thiserror](https://docs.rs/thiserror/latest/thiserror/).

You can also use the `From` / `Into` traits to automatically convert "built-in" types to your custom types. See: https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html

### 18.5.3 Boxing errors

Similar to using `From` / `Into` directly, but `Box`ed into a `dyn`amically dispatched `Error` type.

### 18.5.4 Other uses of ?

`?` doesn't actually mean "`unwrap` or `return Err(err)`".

It more closely resembles "`unwrap` or `return Err(From::from(err))`". This `From` conversion can clean up code required to convert between error types.

### 18.5.5 Wrapping errors

This is "using `From` / `Into` directly", as discussed above.

## 18.6 Iterating over Results

When iterating over a list of operations that can fail, you can...

- ignore failed items
- collect failed items in a separate `mut`able vector
- fail the entire operation
- separate successes from failures in one operation with `partition`

...it just depends how you want to handle the failures.
