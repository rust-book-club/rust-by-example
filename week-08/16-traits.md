# Chapter 16: Traits

A `trait` is a collection of methods.

A type which implements a `trait` gains those methods on its instances.

Within a `trait`
- `Self` is an alias for the name of the trait (e.g. `Animal` in this example).
- `self` is the instance of the object which implements the trait
- `&self` is shorthand for `self: &Self`
- `&mut self` is shorthand for `self: &mut Self`

## 16.1 Derive

The `#[derive(X)]` annotation can be applied to some `struct` or `enum` type to automatically derive some trait `X` for a type.

`derive` is _not_ "compiler magic". It's a macro (code that writes code). You can [write your own macros to automatically derive your own traits](https://doc.rust-lang.org/reference/attributes/derive.html).

## 16.2 Returning Traits with dyn

Important point: "The Rust compiler needs to know how much space every function's return type requires."

This is because of how data is arranged on the stack. Rust needs to know how big some data on the stack is so it can quickly access different things. [See here](https://users.rust-lang.org/t/why-does-rust-need-to-know-the-size-of-types-at-compile-time/67356) and [here](https://stackoverflow.com/a/40115371) for more info.

"Trait objects" are defined with the `dyn` keyword, which is short for _dynamic dispatch_. `dyn MyTrait` means "some type which implements `MyTrait`. Since "some type" has a variable size, we `Box` this `dyn MyTrait` instead; this produces a pointer, `Box<dyn MyTrait>`, which has a definite size.

[Quick explanation of static vs. dynamic dispatch.](https://reintech.io/blog/understanding-implementing-rust-static-dynamic-dispatch)

Note that `Box` is a particular kind of ["smart pointer"](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) in Rust. When we want to add something to the heap, we've been using `Box` throughout this book, but we could also be using `Rc` or `Arc` or any other smart pointer type. You can [create your own smart pointers](https://medium.com/@zainalpour_79971/rust-from-scratch-smart-pointers-2951b7725760), as well.

## 16.3 Operator Overloading

[A handful](https://doc.rust-lang.org/core/ops/) of operators can be overloaded in Rust, mainly arithmetical, logical, indexing, bit shift, and range operators.

## 16.4 Drop

`Drop` defines only one method, `drop`. `drop` is called on any object which implements `Drop`, when that object goes out of scope. This is how the object "cleans up" when it is destructed.

Why would you want to implement `Drop`? Maybe you want to close a file or a database connection, print a message to the user, or do some other cleanup when your object is destroyed.

## 16.5 Iterators

`Iterator` also defines only a single method: `next`.

`next` returns an `Option<Self::Item>` (remember associated types?), which is `None` when the iterator has run out of items.

## 16.6 impl Trait

Use `impl Trait`

- in function arguments, when you don't care about the specific type (just that it implements some trait)
- in function return types, for the same reason

This is similar to `trait` usage in function signatures in Scala.

You might be asking yourself: "so [what's the difference between returning `Box<dyn Trait>` and `impl Trait`](https://users.rust-lang.org/t/difference-between-returning-dyn-box-trait-and-impl-trait/57640/3)?"

The answer is that the type of the object returned by `Box<dyn Trait>` is _unknown_ at compile time. It is determined dynamically at runtime. It uses _dynamic dispatch_.

However, the type behind `impl Trait` _is known at compile time_ but is intentionally hidden by the programmer, usually to make programs more readable, and hide unneeded information. An `impl Trait` uses _static dispatch_.

`Box<dyn Trait>` could be used in place of `impl Trait`, but not the other way around.

`impl Trait` is also used for closures, which have ephemeral, unnameable types (as mentioned a few chapters ago). 

## 16.7 Clone

You may be thinking: ["what's the difference between `Clone` and `Copy`?"](https://stackoverflow.com/q/31012923)

- Think of `Copy` like literally copying the bytes in memory that represent an object. It's easy and should be cheap.
- `Clone` is more expensive, may require some setup; it can be "arbitrarily complicated".

Primitives implement `Copy`, but not `Clone`. `Clone` implies "move semantics", `Copy` implies "copy semantics". `Clone` is explicit, `Copy` is implicit.

## 16.8 Supertraits

Maybe the closest Rust gets to inheritance?

Note the `&dyn` in the function signature in this example. [This could also be `&impl`](https://www.reddit.com/r/rust/comments/eqmn0o/comment/feuaxbm)

## 16.9 Disambiguating overlapping traits

Use _fully qualified syntax_ to disambiguate between methods with the same name implemented by multiple traits on a given type.
