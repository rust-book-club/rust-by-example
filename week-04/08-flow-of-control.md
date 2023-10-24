# Chapter 8: Flow of Control

## 8.1: if/else

No parentheses around `if` conditions in Rust.

Like in Scala, `if`-`else` in Rust is an expression and returns a value.

In Scala, the compiler finds the narrowest common superclass of the types of the branches

```scala
scala> val thing = if (true) Seq(1) else Set(1)
val thing: scala.collection.immutable.Iterable[Int] with Int => AnyVal with Equals{def iterableFactory: scala.collection.IterableFactory[[_]scala.collection.immutable.Iterable[_] with Int with _ => Any with Equals]} = List(1)
```

Rust does not do ([this kind](https://doc.rust-lang.org/beta/reference/type-coercions.html#coercion-sites)) of coercion, and so the programmer must ensure the branches are the same type.

## 8.2: loop

`loop {}` is an infinite loop in Rust.

- skip to the next iteration with `continue`
- exit the `loop` with `break`

## 8.2.1: Nesting and labels

Use a `'label` to break out of a nested loop.

Don't confuse this syntax with `'a` lifetime.

## 8.2.2: Returning from loops

Return some value `x` from a `loop` with `break x`.

Why would you want to do this?

- when connecting to a database / establishing a network connection
- when running a test (like a Scala `eventually`)

## 8.3: while

Note, like `if`, no parentheses around condition.

`while` is a statement in Rust, it evaluates to `()`.

## 8.4: for and range

`a..b` is exclusive range `[a, b)`, `a..=b` is an inclusive range `[a, b]`.

Ranges create iterators. You can also create an iterator from a vector, like

```rs
let names = vec!["Bob", "Frank", "Ferris"];
let iter = names.iter(); // immutably borrows the elements of names
let iter = names.iter_mut(); // mutably borrows the elements of names
let iter = names.into_iter(); // consumes the vector, transforming it into an iterator
```

Use `for elem in elems {}` notation to iterate over the elements of `elems`.

By default, `into_iter()` is assumed if no explicit conversion to an iterator is made.

## 8.5: match

We've seen this a few times already in this book.

Notable difference from Scala: you can pattern match on a range, like `13..=19 => ...`.

`match` is an expression in Rust, so

- it evaluates to some value
- all match arms must evaluate to the same type of value

### 8.5.1: Destructuring

Note: there is [a Rust REPL called `evcxr`](https://github.com/evcxr/evcxr/blob/main/evcxr_repl/README.md) which I use to test small things like the following examples.

#### 8.5.1.1: tuples

In Scala, we have to do the following to ignore many values

```scala
val tuple = (1, false, "hello", 42f, 19.0, 2112L)

tuple match {
  case (1, _, _, _, _, _) => println("_1 is 1")
  case (_, _, _, _, _, 2) => println("_6 is 2")
  case (3, _, _, _, _, 4) => println("_1 is 3 and _6 is 4")
}
```

In Rust, `..` is a shorthand for `_, ` repeated

```rs
let tuple = (1, false, "hello", 42f32, 19.0, 2112i64);

match tuple {
    (1, ..)    => println!(".0 is 1"),
    (.., 2)    => println!(".5 is 2"),
    (3, .., 4) => println!(".0 is 3 and .5 is 4"),
    _          => println!("match is always exhaustive in Rust")
}
```

#### 8.5.1.2: arrays/slices

`..` can also be used to ignore multiple elements in an array.

`_` and `@` work similarly to Scala pattern matching.

#### 8.5.1.3: enums

Identical to Scala.

#### 8.5.1.4: pointers/ref

Pattern matching on references is a bit more involved.

- each arm of a `match` can match on a reference `&`
- or, you can dereference (`*`) and _then_ match

Create a reference from a value with `ref` / `mut ref`.

This is all a bit too complex for where we are right now. So let's ignore this for the moment.

#### 8.5.1.5: structs

You can also use `..` in structs, to ignore fields you don't care about.

Note that, in enums and tuples and structs, `..` must always be unambiguous in pattern matches.

### 8.5.2: Guards

`if` in match arms -- exactly like in Scala.

### 8.5.3: Binding

`@` in patterns -- exactly like in Scala.

## 8.6: if let

`if let` is a Rust construct which allows you to combine a `match` and a destructuring in a single statement.

You can use `else` with `if let`.

**Question for class**

How do we fix the challenge at the bottom of 8.6?

Here's how I did it

```rs
if let Foo::Bar = a {
    // ...
```

## 8.7: let-else

"refutable pattern" -- what's that?

[When a pattern might fail to match](https://doc.rust-lang.org/book/ch18-02-refutability.html), it's called "refutable".

```rs
if let Some(42) = x {
    // this is a refutable pattern, it can fail to match
    // ex. when x is None
}
```

```rs
if let x = Some(42) {
    // this is an irrefutable pattern, it can never fail
    // x will always be bound to Some(42) in this block
};
```

A `let else` expression conditionally binds a value to a variable. When it fails, it `panic`s, or `return`s, or `break`s.

Note: `panic`s are kind of like `Exception`s in Java.

## 8.8: while let

Iterators return `Some` as long as they have more elements to provide, `while let` is useful for processing them.