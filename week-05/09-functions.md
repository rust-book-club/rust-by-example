# Chapter 9: Functions

["Forward declarations" in C.](https://en.wikipedia.org/wiki/Forward_declaration)

Note that C and C++ are very similar languages, sometimes referred to collectively as "C/C++", but [C++ is not a strict superset of C](https://stackoverflow.com/questions/1201593/where-is-c-not-a-subset-of-c). They are pretty different, and the term "C/C++" offends some purists.

Main points for functions

- `fn` instead of `def` / `function` / `fun`
- `-> <return type>`, unlike `: <return type>` in Scala
- default return type is `()` (unit), if no explicit type specified

## 9.1: Associated functions & Methods

"Associated functions" are like Java's `static` functions.

- in Java / Scala, we would call a static function with syntax like `MyClass.function()`
- in Rust, we use double-colons instead: `MyClass::function()`

"Methods" are like instance functions. Any function which takes `self` as a parameter is a method.

In Java, Scala, and Rust, we use the dot operator for methods: `myInstance.method()`.

The differences between `self`, `&self`, and `&mut self` are finally explained here.

Check out the `TODO`s in `main()` here: they give examples of what you can and cannot do with mutable instances.

## 9.2: Closures

**WARNING** The remaining sections in this chapter delve into some _quite difficult_ concepts. Pay close attention.

Closures _close over_ (capture) some part of their environment.

Similar to closures in Scala, but type inference is more powerful (inferred from usage).

Use `||` instead of `()` to hold arguments to a closure.

### 9.2.1: Capturing

Iterating over a collection (`.iter()`, `.into_iter()`, `iter_mut()`), which we discussed previously, requires you to specify what _kind_ of iteration you want to do -- do you want to _own_ the values? Or borrow them? Or borrow them mutably?

Closures try to do a bit more inference -- they capture variables as immutable references, and only move to mutable references, or ownership, when required.

The example in this section touches on a lot of borrow-checker stuff. Let's walk through it...

### 9.2.2 As input parameters

Passing a closure to a function as an argument is more involved in Rust than it is in Scala.

When created, closures can infer ownership, borrowing, or mutable borrowing of captured values. But when a closure is an argument to another function, that behavior must be specified explicitly in the function signature.

- `Fn` is a trait for a function which captures all of values by immutable reference
- `FnMut` captures at least one of its values by mutable reference
- `FnOnce` takes ownership of at least one of its captured values, and therefore can only be called once

When `Fn`, `FnMut`, and `FnOnce` are used as argument types, think of them as _lower bounds_ for what could possibly be passed in. A parameter whose type is an `FnOnce` could be passed an `FnOnce` argument, an `FnMut` argument, or an `Fn` argument. A parameter whose type is an `FnMut` could be passed an `FnMut` or an `Fn` argument. But an `Fn` parameter can only take an `Fn` argument.

Think of it like: **"what's the worst that could happen?"**. An `FnMut` argument _could_, at worst, take a mutable reference to a captured value, but it cannot take ownership. (It could also take an immutable reference.)

**VERY IMPORTANT NOTE** the "flavour" of a closure does not relate to the _arguments_ to the closure. It relates to the _captured values_. An `FnMut` closure cannot take ownership of _captured values_, but it can take ownership of _arguments passed to the closure_. Confused? [You're not alone.](https://stackoverflow.com/questions/56743984/ownership-closures-fnonce-much-confusion)

See: `closures` cargo project in this directory.

All closures implement `FnOnce`. They can all be called at least once.

> Tip: When specifying the type of a _closure_, be as _conservative_ as possible. Don't take ownership of a value if you only need a reference. When specifying the type of a _closure parameter_, be as _liberal_ as possible. Allow users to pass in `FnOnce` and `FnMut` arguments, unless there's a particular reason why you want to restrict the parameter to `Fn` arguments only.

[_The Rust Programming Language_](https://doc.rust-lang.org/book/ch13-01-closures.html) has a great chapter on closures.

### 9.2.3: Type anonymity

This section very verbosely says we need to use generics when defining closure parameters.

Note that we can use [alternate syntax](https://stackoverflow.com/a/47515540) here if we don't care about the return type of the closure

```rs
fn apply<F>(f: F) where
    F: Fn() {
    f();
}
```

is equivalent to

```rs
fn apply(f: impl Fn()) {
    f();
}
```

We can't specify the type in the second example, though

```rs
fn foo<T: Trait>(t: T)
fn bar(t: impl Trait)

foo::<u32>(0) // this is allowed
bar::<u32>(0) // this is not
```

Also, we've seen this `where` syntax a bit, but it hasn't been explained yet. It mainly [makes specifying type parameter bounds a bit more ergonomic](https://rust-lang.github.io/rfcs/0135-where.html#summary).

### 9.2.4: Input functions

Functions can be passed as arguments to other functions with closure parameters.

Note that we also have _function pointers_ in Rust, which [are not closures](https://hashrust.com/blog/a-guide-to-closures-in-rust/#:~:text=Unlike%20a%20closure%20trait%20a%20function%20pointer%20type%20is%20not%20a%20trait.&text=There%20are%20times%20when%20using,function%20pointer%20can%20be%20performance.).

There are a lot of nuances here that are beyond the scope of this course.

### 9.2.5: As output parameters

This section answers the question: "can I return a closure from a function"?

The answer is: "yes, but it's not intuitive at all".

I haven't done this too much yet, so this is new to me, as well.

### 9.2.6: Examples in std

Examples from the standard library.

#### 9.6.2.1: Iterator::any

Why `FnMut` and not just `Fn`? See: https://stackoverflow.com/a/58465588 and https://stackoverflow.com/a/69463173

In short: an `FnMut` can also mutate _itself_, so if you want a closure with some internal mutable state, you need `FnMut`.

(But remember: wherever we can pass an `FnMut` as an argument, we can also pass an `Fn`.)

#### 9.2.6.2: Searching through iterators

`.position()` is like `.indexOf()`

## 9.3: Higher Order Functions

Functions which take functions as arguments... we've already covered this quite a bit.

Allows the functional / lambda approach we're familiar with from Java 8+ and Scala.

## 9.4 Diverging functions

The `!` type... like the `Nothing` type in Scala. (The type of `throw` expressions.)

In Rust, the `continue` keyword, `panic!()` macro, `loop {}` expressions, etc. all have type `!`.