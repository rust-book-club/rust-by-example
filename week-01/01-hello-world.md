# Chapter 1: Hello World

## 1.1: Comments

"Doc comments... are parsed into HTML library documentation"

```rs
fn outer_fn() {
  //! Generate library docs for the enclosing item (outer_fun).

  /// Generate library docs for the following item (inner_fun).
  fn inner_fun() {
    todo!()
  }
}
```

Typically, we use `//!` to document modules and crates, and `///` to document everything else. See: https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html

Block comments can be nested in Rust: `/* /* nested */ */`.

## 1.2: Formatted print

Not too dissimilar to other languages. Note

- `print` / `println` print to stdout
- `eprint` / `eprintln` print to stderr
- `format` writes formatted text to a `String`

Note the `!` at the end of these method calls, though. These are _macros_. In Rust, the `print` methods are macros to allow them to accept any number and type of arguments. (Rust [does not support _variadic generic_ arguments](https://www.reddit.com/r/rust/comments/jrlf7p/why_doesnt_rust_have_varargs/) without the aid of macros.)

Rust allows types to implement two separate traits for display

- `fmt::Display` -- typically used to display something to the user; invoked with the `{}` marker
- `fmt::Debug` -- typically used to give all the information you have about an object, for debugging purposes; invoked with the `{:?}` marker

...this allows for better separation of concerns.

Note that we don't get a `toString()` method "for free" in Rust, like we do in Java and Scala. We need to implement the `Display` trait for our type in order to be able to insert it into a string with `{}`.

**Question for class**

Did anyone try running `rustc --explain E0277` locally? (Rust also has an [Error codes index](https://doc.rust-lang.org/error_codes/E0277.html) available online, which explains every error the compiler emits.)

Note that `#[allow(dead_code)]` is an _attribute_ in Rust, similar to _annotations_ in Java and Scala. (In Scala, [the equivalent annotation](https://www.scala-lang.org/2021/01/12/configuring-and-suppressing-warnings.html) would be `@nowarn(cat=w-flag-dead-code)`.)

**Question for class**

Was anyone able to solve the third activity?

Here's my solution:

```rs
let pi = 3.141592;
println!("Pi is roughly {:.3}", pi);
```

### 1.2.1: Debug

As mentioned above, we need to implement the `Display` / `Debug` traits on our types in order for them to be displayable. Thankfully, this is pretty easy in most cases

```rs
#[derive(Debug)]
struct DebugPrintable(i32);
```

The `derive` attribute attempts to automatically derive a particular trait for a particular type. It can be used for all sorts of traits, not just `Debug`.

You can also manually implement traits (including `Debug`) for your custom types. We'll cover this in Chapter 16.

### 1.2.2: Display

**Question for class**

Who completed the activity in this section?

Here's my solution:

```rs
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    // ...
    let complex = Complex { real: 3.3, imag: 7.2 };

    println!("Compare points:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
```

The output looks like

```
Compare points:
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
```

#### 1.2.2.1: Testcase: List

**Question for class**

The wording here is a bit confusing... _why_ do we need `?`?

It's because `write` returns a `Result` type -- it can fail. If displaying any element of the vector fails, we don't want to write only _some_ of it, we want to fail the whole thing.

[The question mark operator (`?`) unwraps valid values or returns erroneous values"](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator), so if writing any element fails, `fmt` will return some `Error` type.

This is equivalent to something like the following in Scala

```scala
write(f, "[") match {
  case Success(value) => value
  case Failure(exception) => return Failure(exception)
}
```

...so you can see how it would cause an early return in the case of a failure.

**Question for class**

How do we solve the activity here?

Here's my solution:

```diff
- write!(f, "{}", v)?;
+ write!(f, "{}: {}", count, v)?;
```

### 1.2.3: Formatting

Similar to formatting strings in other languages, like C.

Not worth memorizing these -- [look them up](https://doc.rust-lang.org/std/fmt/) when you need them.

**Question for class**

How did you solve the activity from this section?

Here's my solution:

```rs
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue)?;
        write!(f, " 0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
```