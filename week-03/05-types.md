# Chapter 5: Types

## 5.1: Casting

> "Rust provides no implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the as keyword."

In general, just _be careful_.

## 5.2: Literals

Literal suffixes can be used to specify primitive types, like `3f32`.

Explicit type annotations can also be used.

If no explicit type annotation or literal suffix exists, the Rust compiler will infer the type from usage.

Default integral type is `i32` (equivalent to Java `Int`).

Default floating point type is `f64` (equivalent to Java `Double`).

## 5.3: Inference

Types are inferred based on usage; much more sophisticated than Java / Scala.

## 5.4: Aliasing

Similar to type aliases in Scala.

Type aliases must be `PascalCase` (aka. `UpperCamelCase`).

This is to [keep code style consistent](https://www.reddit.com/r/rust/comments/y361wc/why_rust_prevent_camelcase_variables_by_default/) across codebases, increasing readability.
