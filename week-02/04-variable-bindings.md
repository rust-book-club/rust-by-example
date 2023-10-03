# Chapter 4: Variable Bindings

Note: compiler warnings are not shown in-browser.

["binding" vs "assignment"?](https://www.reddit.com/r/ProgrammingLanguages/comments/c3yr79/assignment_vs_variable_binding/)

Some Rust resources make a point of distinguishing the two, so be aware.

For more on underscore-prefixed variables, see: https://stackoverflow.com/a/48361729

## 4.1: Mutability

`let` bindings are immutable by default.

> Programming language design note: `let mut` requires a bit more thought (and typing) and so gently discourages mutability. Compare this to `val` and `var` in Scala, in which (1) can be easily confused and (2) are equally as easy to type. This can lead to subtle bugs, and bad practices.

## 4.2: Scope and Shadowing

Note that shadowed variables can be of different types. (This includes [making mutable variables immutable](https://www.thorsten-hans.com/shadowing-temporary-mutability-rust/), as we'll see in Section 4.4.)

This is useful for, e.g. unwrapping `Option`al values, `Result`s, etc. (No `maybeThing: Option[Thing]` and `thing: Thing` like in Scala.)

## 4.3: Declare first

**Question for Class**

Can anyone think of a case in which you might _not_ want to initialize a variable when it's declared? Even with a default value?

In resource-limited systems, when creating many variables, [default values may be a waste of resources](https://www.learncpp.com/cpp-tutorial/uninitialized-variables-and-undefined-behavior/).

## 4.4: Freezing

See: https://www.thorsten-hans.com/shadowing-temporary-mutability-rust/