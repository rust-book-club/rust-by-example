# Chapter 10: Modules

Similar to packages in JVM languages.

Use `mod` to create a module.

## 10.1: Visibility

Everything is private by default. Use `pub` to make things public.

Like [Scala's access modifiers](https://alvinalexander.com/scala/how-to-control-scala-method-scope-object-private-package/), Rust has the ability to make items visible only to selected scopes.

- `pub(in crate::<...>)`
- `pub(self)` == default (private) visibility
- `pub(super)`

## 10.2: Struct visibility

Similar to classes in Java / Scala, structs in Rust can have _both_ `pub`lic and private fields.

## 10.3: The use declaration

Rust's `use` and `as` are like `import` and `=>` (Scala 2) / `as` (Scala 3) in Scala.

## 10.4: super and self

Like `super` and `this` in Scala.

`crate` is kind of like [`_root_` in Scala](https://stackoverflow.com/a/9281415).

## 10.5: File hierarchy

Someone asked me recently how to structure a larger Rust project. This section explains some of that.