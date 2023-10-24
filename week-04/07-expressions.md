# Chapter 7: Expressions

Expressions return a value, statements return `()` (unit).

**Question for class**

What do you think this will print?

```rs
let mut x;
let mut y;

x = y = 1;

println!("{:?} {:?}", x, y);
```

See https://stackoverflow.com/a/75125942 for an explanation.

Variable binding / declaration is a statement; it returns `()`.

Turn any expression into a statement by adding `;` to the end of it.