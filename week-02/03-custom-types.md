# Chapter 3: Custom Types

ADTs in Rust:

- `struct`, a product type (like a Scala 2* `case class`)
- `enum`, a sum type (like a Scala 2* `sealed trait`)

> * Note that Scala 3 has [explicit support for ADTs with the `enum` keyword](https://docs.scala-lang.org/scala3/book/types-adts-gadts.html#algebraic-datatypes-adts).

`const` vs `static`? We'll discuss this in 3.3.

## 3.1: Structures

Types of structs

- classic C-style structs (`struct Point { x: f32, y: f32 }`)
- tuple structs (`struct Pair (i32, i32)`)
- unit structs (`struct Flag;`)

"Newtypes"

```rs
struct Currency(f32);

struct Weight(f32);

// does this function take a currency or a weight?
fn pounds(value: f32) -> Unit {
    todo!()
}
```

**Question for class**

Anyone solve Activity 1?

Here's my solution:

```rs
    fn rect_area(rect: Rectangle) -> f32 {
        let Rectangle {
          top_left: Point { x: x1, y: y1 },
          bottom_right: Point { x: x2, y: y2 }
        } = rect;

        (x2 - x1) * (y1 - y2)
    }
    
    let p1: Point = Point { x: 10.3, y: 0.4 };
    let p2: Point = Point { x: 15.3, y: 0.0 };
    let rect = Rectangle { top_left: p1, bottom_right: p2 };
    
    // prints: area of rect: 2
    println!("area of rect: {}", rect_area(rect));
```

How about Activity 2?

Here's my solution:

```rs
    fn square(top_left: Point, side_length: f32) -> Rectangle {
        Rectangle {
          top_left: Point {
            x: top_left.x,
            y: top_left.y
          },
          bottom_right: Point {
            x: top_left.x + side_length,
            y: top_left.y - side_length
          }
        }
    }
    
    let top_left = Point { x: 6.0, y: 6.0 };
    let side_length = 5.0;
    let square = square(top_left, side_length);
    
    println!("area of square: {}", rect_area(square));
```

**Question for class**

Could we just re-use `top_left` inside of `fn square`, like

```rs
    fn square(top_left: Point, side_length: f32) -> Rectangle {
        Rectangle {
          top_left: top_left,
          bottom_right: Point {
            x: top_left.x + side_length,
            y: top_left.y - side_length
          }
        }
    }
```

...? Did anyone try this?

## 3.2: Enums

Variants of an `enum` can be any kind of `struct`.

Note the `Self` alias -- this is used a lot in `impl` blocks.

```rs
impl ExampleType {
    fn foo(self: Self) -> Self {
        todo!()
    }

    fn bar(&self) -> f32 {
        todo!()
    }

    fn baz(&mut self) -> f32 {
        todo!()
    }
}
```

- `todo!()` is a convenient macro to stub out as-yet-unimplemented code
- `Self` is an alias for `ExampleType` inside the `impl` block
- `self: Self` can be shortened to just `self`
    - the `Self` type means the method takes ownership of `self`
- `&self` is shorthand for `self: &Self`
    - the `&Self` type means the method borrows `self` but cannot mutate it
- `&mut self` is shorthand for `self: &mut Self`
    - the `&mut Self` type means the method borrows `self` and can mutate it

Usually, an `impl` method which takes ownership of `self` returns a new value of type `Self`. This is similar to [a fluent interface in Java / Scala](https://en.wikipedia.org/wiki/Fluent_interface#Scala).

### 3.2.1: use

(Notice the `#![allow(dead_code)]` (`#![...]`) internal attribute at the top of this file.)

`use` is like `import` in Scala.

Best practice is to _not_ do what's being done here in this example. Prefer using `Status::Poor` and `Status::Rich` to a `use crate::Status::*` statement.

You might also see `mod` in some Rust code, this is [similar, but slightly different](https://users.rust-lang.org/t/why-would-rust-use-both-mod-and-use/77045/6) to an `import` statement in Scala.

`mod` only needs to be used once, anywhere in the project. It tells the Rust compiler that you want to bring that module into your current crate (project).

See: https://stackoverflow.com/a/63201764

### 3.2.2: C-like

Notice the "implicit discriminator" comment.

You can also specify an explicit discriminator.

Try setting `Zero = 42`.

### 3.2.3: Testcase: linked-list

**Question for class**

What is this `Box` type? Anyone have any thoughts?

`fn new()` is a common way to provide constructor-like functionality of Rust data types.

Note the `after Rust 2018` note -- we are past this. You can get rid of the `*` on `*self` and the `ref` on `ref tail` and the example will still work.

**Question for class**

Anyone know of any other common data types which can be implemented using only `struct`s and `enum`s?

## 3.3: constants

What's the difference between `const` and `static`?

- `static` exists in a single place in memory throughout the lifetime of the program
    - `static`s can be `mut`able
    - global mutable data is inherently `unsafe`
- `const` is inlined and may not always refer to the same memory location
    - `const`s are always immutable

When to use `const` vs `static`?

[Default to using `const`, and move to immutable `static`s only if necessary.](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/const-and-static.html)
