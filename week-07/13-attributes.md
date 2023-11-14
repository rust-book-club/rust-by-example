# Chapter 13: Attributes

> When attributes apply to a whole crate, their syntax is `#![crate_attribute]`, and when they apply to a module or item, the syntax is `#[item_attribute]` (notice the missing bang `!`).

> Attributes can take arguments with different syntaxes:
>
> `#[attribute = "value"]`
> `#[attribute(key = "value")]`
> `#[attribute(value)]`

They can take multiple values and be split over multiple lines.

[Common attributes include](https://dhghomon.github.io/easy_rust/Chapter_52.html)

- `#[allow(x)]` where `x` can be `dead_code`, `unused_variables`, etc.
- `#[derive(x, y, z)]` where `x`, `y`, `z` are traits like `Display`, `Eq`, `Ord`, etc.
- `#[cfg(test)]` and `#[test]` used when writing tests
- `#![no_std]` to not bring in Rust's standard library when it's not needed

Huge list of attributes [available here](https://doc.rust-lang.org/reference/attributes.html).

## 13.3 cfg

`cfg` allows conditional compilation of code.

Useful if producing different binaries for different target architectures / OSes.

Also useful for defining test modules, e.g.

```rs
fn return_two() -> i8 {
    2
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_returns_two() {
        assert_eq!(super::return_two(), 2);
    }
}
```