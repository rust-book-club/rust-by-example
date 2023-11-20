# Chapter 15: Scoping Rules

## 15.1 RAII

RAII == Resource Acquisition is Initialization

What this means is that acquiring "resources" (space on the heap) is equivalent to initializing (constructing) an object. You cannot use an object in a program before you have the resources it needs to do whatever it needs to do.

The opposite is also true, object destruction ("finalization") is equivalent to freeing those resources. Once the resources are gone, the object is gone.

In Rust, objects on the heap are destroyed (and their memory is freed) when they go out of scope.

Note that you cannot manually call `.drop()` on an object, but you can use `std::mem::drop(obj)` [(see here)](https://doc.rust-lang.org/std/mem/fn.drop.html).

## 15.2 Ownership and moves

Any variable which owns resources is responsible for freeing those resources, as well. So resources can only have one owner.

Simple data with known size (including primitives) are stored on the stack.

Data with variable size (like collections) are stored on the heap.

> See: [What and where are the stack and heap?](https://stackoverflow.com/a/80113)

Stack data is usually easy and cheap to copy, so ["move semantics"](https://stackoverflow.com/q/30288782) do not apply.

"Move" in Rust doesn't mean to literally move bytes around in RAM. It means to change the owner of a variable in code.

The example on lines 18-19 here...

```rs
// `a` is a pointer to a _heap_ allocated integer
let a = Box::new(5i32);
```

...shows "boxing" an integer. This is very similar to e.g. ["boxing" an `int` into an `Integer` in Java](https://docs.oracle.com/javase/tutorial/java/data/autoboxing.html).

### 15.2.1 Mutability

> "Mutability of data can be changed when ownership is transferred."

**Question for class**: Why do we need the `*` (asterisk) before the reassignment in this example?

### 15.2.2 Partial moves

It's possible that _some_, but not _all_ of the data of an object is moved somewhere else.

In that case, referencing the object itself becomes invalid, but referencing the unmoved fields is still valid.

This is a bit of an unusual case.

## 15.3 Borrowing

Why borrow? Big objects, stored on the heap, are expensive to copy over and over. So instead of doing that, we just point to them with references. But if the object is destroyed, anything pointing to it is no longer valid, so Rust statically analyzes code and does not allow references to exist to objects after they've been destroyed.

### 15.3.1 Mutability

`let` is like `val` in Scala; `let mut` is like `var`.

Similarly, a `&` reference to a variable is immutable, but a `&mut` reference lets you mutate the data you have a reference to.

You cannot borrow an immutable (`let`) object as `&mut`, but any other combination is allowed.

### 15.3.2 Aliasing

Any number of `&` immutable references are allowed in any scope.

But if an `&mut` mutable reference exist to an object, _no other references_ (even immutable ones) can exist.

Because `&` immutable references only have read access to data, it's safe to have any number of them. But `&mut` references have read-write access, if any other references existed (even only read-only ones), it would be possible to create data races, leading to potentially unpredictable behaviour.

Only once the `&mut` reference is no longer used (determined by static analysis) can other references exist. Note that the reference doesn't necessarily need to be "destroyed" or even go out of scope, it just needs to not be used at any latter line in the program. See [this explanation](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-borrow-checker) of the static analysis done by the Rust compiler to determine a mutable reference's _lifetime_.

### 15.3.3 The ref pattern

I'm sure there are places where it's useful, but this example is a bit contrived to me.

## 15.4 Lifetimes

A _variable's_ lifetime starts at construction and ends at destruction.

But a _reference's_ lifetime starts at its declaration site and ends when it is used for the last time. As mentioned above, this is not necessarily the same as when it goes out of scope. It is less than or equal to that duration.

In the example here, we can get rid of the two inner scopes and everything still works. In fact, we can make the two borrows _mutable_ and everything still works... because the lifetimes of the two references do not overlap.

### 15.4.1 Explicit annotation

Lifetimes are a new kind of generic parameter.

Like generic type parameters, generic lifetime parameters appear in the `<>` angle brackets of `struct`, `fn`, etc. declarations.

While the idiom for type parameters is uppercase single letters like `T` and `U`, the idiom for lifetime parameters is lowercase single letters like `'a` and `'b`. Lifetime parameters always begin with an apostrophe `'`.

Think about how we use generic type parameters -- a function might take a `T` and return a `T`, or it might add bounds to a type parameter like `T : Display`. We can do the same with lifetime parameters -- a function might take a reference with some lifetime `'a` and return a value with the same lifetime `'a`.

`'static` is the default (unconstrained) lifetime. `'static` means that a variable is alive as long as the program is running.

In the example here, "the lifetime of `y`" is explicitly declared to be `'a`, which is unconstrained by any function argument, and so defaults to `'static`. Getting rid of the `'a` and declaring `y` as `let y: &'static i32 = &_x;` has the same effect.

Note that in many cases (and unlike generic type parameters), [lifetime parameters can be elided (omitted)](https://doc.rust-lang.org/nomicon/lifetime-elision.html). You can remove the `'a` and `'b` from `print_refs` and it still works just fine.

### 15.4.2 Functions

> Ignoring elision, function signatures with lifetimes have a few constraints:
> - any reference _must_ have an annotated lifetime.
> - any reference being returned _must_ have the same lifetime as an input or be `static`.

But lifetime elision often makes this much less verbose.

### 15.4.3 Methods

Identical to non-method functions.

### 15.4.4 Structs

Lifetime annotations appear on the `struct` itself, and are used by the `struct`'s fields

```rs
enum Either<'a> { // <- lifetime parameter declared here
    Num(i32),
    Ref(&'a i32), // <- lifetime parameter used here
}
```

Beginner Rust tutorials often use `String` (uppercase `S`) strings in early examples, or avoid strings altogether, since string literals (like `"hello"`) are of type `&str` and require lifetime annotations. We hit this issue [here in Chapter 14](https://doc.rust-lang.org/rust-by-example/generics/bounds/testcase_empty.html) before this book even mentioned lifetimes.

### 15.4.5 Traits

The syntax here is

```rs
struct Borrowed<'a> {
    x: &'a i32,
}

impl<'a> Default for Borrowed<'a> {
    // ...
}
```

In all places where `'a` appears, it refers to the same generic lifetime. Remember that generic _type_ parameters work in a very similar way

```rs
struct Owned<T> {
    x: T,
}

impl<T> Default for Owned<T> {
    // ...
}
```

### 15.4.6 Bounds

> Just like generic types can be bounded, lifetimes (themselves generic) use bounds as well. The `:` character has a slightly different meaning here, but `+` is the same. Note how the following read:
> 1. `T: 'a`: _All_ references in `T` must outlive lifetime `'a`.
> 2. `T: Trait + 'a`: Type `T` must implement trait `Trait` and _all_ references in `T` must outlive `'a`.

The example in this section is quite small, but quite complex.

### 15.4.7 Coercion

Lifetimes can be coerced to be shorter, but not longer, in order to match some signature.

### 15.4.8 Static

The `'static` lifetime means a reference is valid for the remainder of the program.

`static` constants and (by default) string literals have `'static` lifetime.

### 15.4.9 Elision

As mentioned a few times now, _lifetime elision_ allows us to omit some lifetime annotations.

Read more about elision in [The Rust Programming Language](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision).
