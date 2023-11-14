# Chapter 14: Generics

Similar to Java, Scala.

## 14.1 Functions

Similar to Java, Scala.

## 14.2 Implementation

Generic `impl` blocks have a `<T>` which precedes the name of the `impl`

```rs
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}
```

You can have specialized implementations which only apply to specific `T`s, like

```rs
struct MyType<T> {
    value: T
}

// generic, applies to any T
impl<T> MyType<T> {
    fn new(t: T) -> MyType<T> {
        MyType { value: t }
    }
}

// only applies when T == bool
impl MyType<bool> {
    fn negate(self) -> MyType<bool> {
        MyType::new(!self.value)
    }
}

// only applies when T == i32
impl MyType<i32> {
    fn double(self) -> MyType<i32> {
        MyType::new(self.value * 2)
    }
}
```

## 14.3 Traits

Similar to Java, Scala.

## 14.4 Bounds

`T: Display` is similar to [Scala's type bound syntax](https://docs.scala-lang.org/tour/upper-type-bounds.html) `T <: Display`.

### 14.4.1 Testcase: empty bounds

Empty traits can be used as "marker" traits and can also be used in trait bounds.

## 14.5 Multiple bounds

Saying "type `T` must extend _both_ types `A` and `B`"

```rs
T: A + B
```

This is similar to something like this in Scala

```scala
trait A
trait B

case class C() extends A with B

def thing[T <: A & B](t: T): Unit = {
println(t)
}

thing(C())
```

## 14.6 Where clauses

We covered these earlier in this class.

> When using a `where` clause is more expressive than using normal syntax. The `impl` in this example cannot be directly expressed without a `where` clause

```rs
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // ...
}
```

`T` isn't bound by anything, but `Option<T>` must implement `Debug`.

## 14.7 New Type Idiom

A _newtype_ is a wrapper type around another (usually simpler) type.

It provides some type safety guarantees (e.g. this `String` is a `Username`, but this `String` is a `Password`).

## 14.8 Associated Items

What is an "item"?

[Basically any _thing_ in Rust](https://doc.rust-lang.org/reference/items.html), including modules, `use` declarations, `impl` blocks, and function, type, struct, enum, union, and trait definitions.

### 14.8.1 The Problem

### 14.8.2 Associated types

Make generic data types more ergonomic, as shown in this example.

## 14.9 Phantom type parameters

Similar to ["tagged types" in Scala](https://medium.com/iterators/to-tag-a-type-88dc344bb66c).

Example: A `Velocity` can be treated as a `Double`, but also as a more specific kind of `Double`, with a unit attached.

Zero-sized, erased at compile time (does not appear in the binary).

### 14.9.1 Testcase: unit clarification

Trying to add feet and meters as lengths with phantom unit types.