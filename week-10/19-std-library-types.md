# Chapter 19: Std library types

We've got primitives, `struct`s and `enum`s, but what else is there?

## 19.1 Box, stack, and heap

> "All values in Rust are stack allocated by default. Values can be boxed (allocated on the heap) by creating a `Box<T>`. A box is a smart pointer to a heap allocated value of type `T`. When a box goes out of scope, its destructor is called, the inner object is destroyed, and the memory on the heap is freed."

Use `mem::size_of_val` to see how many bytes some object occupies on the stack. Pointers are 8 bytes.

## 19.2 Vectors

> "Vectors are re-sizeable arrays."

> "A vector is represented using 3 parameters: pointer to the data, length, capacity."

## 19.3 Strings

At the most basic level, a string is an array of bytes. Rust requires strings to contain valid UTF-8 sequences, so [the order of bytes is somewhat restricted](https://stackoverflow.com/a/3886015).

Strings, like any array, can have a fixed size or an unknown size (aka. be a "slice" `&[T]`).

Unknown-size strings can be referred to like any slice, using a reference. As an array of bytes, this type in Rust is `&[u8]`, or, more specifically, `&str` (`&str` [enforces the UTF-8 validity](https://www.reddit.com/r/rust/comments/mvc6o5/comment/gvb69qi); `&[u8]` doesn't, and is sometimes called a "byte string").

A string can be represented by a vector of bytes `Vec<u8>`, as well. This is the data stored by an uppercase `S` `String`. This `Vec<u8>` is stored on the heap and is therefore growable, like any other `Vec`.

`&str` and `String`s both need to care about lifetimes and ownership, because they both have _move semantics_.

**Weird bits**

- Fixed-size strings can be stored as string buffers on the stack, like any fixed-size object: https://docs.rs/arrayvec/0.4.10/arrayvec/struct.ArrayString.html. This implementation implements `Copy` for fixed-size strings, and so these fixed-size strings have copy semantics, like any other stack-stored data

- String literals "are not stored in the heap or the stack, they are stored directly in your program's binary." [source](https://users.rust-lang.org/t/str-string-literals/29635/2)

- `str` is [always](https://www.reddit.com/r/rust/comments/usyvc7/comment/i96jfxb) behind a pointer (`&str` rather than `str`), but it doesn't always have to be a _reference_ `&`. It could be a `Box<str>` or an `Rc<str>`, etc.
    - "you won't have a _stack-allocated_ `str` because it's not a `Sized` type" [source](https://www.reddit.com/r/rust/comments/usyvc7/comment/i96m2dw)
    - See: https://doc.rust-lang.org/std/marker/trait.Sized.html
    - `str` has a [_negative implementation_ of `Sized` -- `!Sized`](https://doc.rust-lang.org/std/primitive.str.html#impl-Sized-for-str)

### Literals and escapes

Escape with `\`, like in Java / Scala (e.g. `\"`, `\\`, etc.)

Use raw string literals to avoid having to escape a bunch of stuff

```rs
// with a regular string literal "..."
let s = "He said \"Hi, my name is...\" and I said \"Slim Shady?\"";
```

```rs
// with a raw string literal r#"..."#
let s = r#"He said "Hi, my name is..." and I said "Slim Shady?""#;
```

> If you need `"#` in your string, just use more `#`s in the delimiter. You can use up to 65535 `#`s.

More about byte strings at the end of this section.

## 19.4 Option

Very similar to `Option` in Scala / `Optional` in Java.

> Unwrapping a `None` variant will `panic!`.

## 19.5 Result

Very similar to `Try` in Scala, but more general: in Scala, `Failure` must contain a `Throwable`; in Rust, `Err` can contain any type `E`.

In the example in this section, we define our own `MathError` `enum`. It doesn't need to extend any base "error trait".

### 19.5.1 ?

As we've seen before...

> "...`?` is used at the end of an expression returning a `Result`, and is equivalent to a match expression, where the `Err(err)` branch expands to an early `return Err(From::from(err))`, and the `Ok(ok)` branch expands to an `ok` expression."

Understanding the example in this section requires a bit of a math refresher.

## 19.6 panic!

The example in this section uses [`valgrind`, a command-line tool](https://valgrind.org/) for memory profiling of applications.

## 19.7 HashMap

Similar to any other language with a map / hash table / dictionary / key-value store type.

> "`HashMap` keys can be [any] type that implements the `Eq` and `Hash` traits."

[API is](https://doc.rust-lang.org/std/collections/struct.HashMap.html): `.insert()`, `.get()`, `.remove()`

### 19.7.1 Alternate / custom key types

> "Note that `f32` and `f64` do _not_ implement `Hash`"

Use `#[derive(PartialEq, Eq, Hash)]` to turn your custom type into a valid `HashMap` key type.

### 19.7.2 HashSet

Similar to any other language with a set / hashset type.

`HashSet`s cannot contain duplicate elements.

[API is](https://doc.rust-lang.org/std/collections/struct.HashSet.html): `.insert()`, `.contains()`, `.union()`, `.difference()`, `.intersection()`, etc.

## 19.8 Rc

`Rc` is a reference-counted (hence "r" "c") smart pointer.

It emulates how object cloning / cleanup works in languages like Scala and Java.

`.clone()` creates a "shallow copy" (i.e. just copies the pointer, not the actual data on the heap).

## 19.9 Arc

`Arc` is atomically reference-counted (threadsafe).

Note the `move` in the example here. `apple` in the inner scope is a shallow copy of `apple` in the outer scope. The inner-scoped `apple` is `move`d into a new thread, so multiple threads are referencing this one object via `Arc`.
