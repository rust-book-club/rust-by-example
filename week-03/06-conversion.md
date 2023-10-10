# Chapter 6: Conversion

Convert between primitive types with `as`.

Convert between custom types (`struct`s and `enum`s) with the `From` and `Into` traits.

## 6.1: `From` and `Into`

"`B` from `A`" implies "`A` into `B`" is also valid.

`String::from` can create a `String` from a `&str[]` string slice.

If the compiler knows the type you're trying to convert to, `a.into()` will convert `a` to the appropriate type by calling `from(a)` on the target type.

## 6.2: `TryFrom` and `TryInto`

Failable versions of `From` and `Into`.

Return `Result<T>` instead of `T`.

## 6.3: To and from Strings

Implement the `Display` trait for a type to automatically get a `ToString` implementation for it.

> See: [automatically-implemented traits and negative implementations](https://doc.rust-lang.org/beta/unstable-book/language-features/auto-traits.html). Also: [this Reddit thread](https://www.reddit.com/r/rust/comments/gwnccy/is_it_possible_to_have_trait_automatically/).

The "turbofish" is a [Rust trope](https://enet4.github.io/rust-tropes/).

Implement `FromStr` on any type to easily parse a type from a string.
