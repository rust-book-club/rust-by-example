# Chapter 2: Primitives

**Question for the class**

Why so many scalar primitive types? Why do you think Rust defines so many when other languages get away with so few? Do you know of any other languages which define many primitive types?

Scala / Java infer types at construction

```scala
var inferred_type = 12
inferred_type = 4294967296 // error: Integer literal is out of range for type int
```

...but Rust infers types based on usage

```rs
let mut inferred_type = 12; // Type i64 is inferred from another line.
inferred_type = 4294967296i64;
```

## 2.1: Literals and operators

"Signed" vs "unsigned" integers

- we can assert integers are non-negative at compile time
- we can use less memory if we don't need negative values

**Question for the class**

Anyone try the `TODO` in this section?

Arithmetic overflow causes a `panic` (similar to a Java `Exception`) when building in debug (default) mode. When building in release mode, values wrap around instead. This is configurable in `Cargo.toml`

## 2.2: Tuples

`too_long_tuple` is reminiscent of Scala 2's `Tuple22` limit. Scala 3 [drops this limit](https://docs.scala-lang.org/scala3/reference/dropped-features/limit22.html).

**Question for class**

Who did Activity 1?

Here's my solution:

```rs
impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}
```

**Question for class**

How about Activity 2?

Here's my solution:

```rs
fn transpose(m: Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}
```

## 2.3: Arrays and Slices

Let's not gloss over this: "their length, which is known at compile time, is part of their type signature `[T; length]`."

This means `[i32; 3]` and `[i32; 4]` are _different types_. Substituting one for the other will cause a _compilation error_. This is a huge departure from Java / Scala.

(Note: this is available in some libraries like [shapeless](https://github.com/milessabin/shapeless), [sized](https://github.com/travisbrown/sized), and [refined](https://github.com/fthomas/refined), but is not available in vanilla Scala 2. It is available in vanilla Scala 3.)