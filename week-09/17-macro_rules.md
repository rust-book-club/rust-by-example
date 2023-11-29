# Chapter 17: macro_rules!

Macros are "code writing code".

`macro_rules!` macros are _declarative macros_.

There are [other kinds of macros in Rust](https://doc.rust-lang.org/book/ch19-06-macros.html#declarative-macros-with-macro_rules-for-general-metaprogramming) including _procedural macros_ which let you write your own attributes (like `#[some_attribute]`).

Why are macros useful?
- DRY
- DSLs
- variadics

## 17.1 Syntax

### 17.1.1 Designators

Macros manipulate source code, so we need ways to refer to blocks, expressions, identifiers, literals, etc. That's what _designators_ are.

### 17.1.2 Overload

The closest Rust gets to overloaded function signatures.

### 17.1.3 Repeat

Enables things like `println!()`, which can take a variable number of arguments ("variadics").

## 17.2 DRY (Don't Repeat Yourself)

In this example, `add_assign`, `mul_assign`, and `sub_assign` all have very similar implementations.

Rather than copying-and-pasting source code to implement them, we can use a macro.

## 17.3 DSL (Domain Specific Languages)

This is a very poor example of a DSL (the `eval` keyword is the entire DSL).

[Here is a much better example](https://blog.rockthejvm.com/akka-streams-graphs/#step-3-glue-the-components-together) of what's possible with a DSL.

## 17.4 Variadics

As disussed in Section 17.1.3.