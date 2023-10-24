#[allow(unused_variables, non_snake_case)]

fn main() {

    //
    // Point 1
    //
    // "A closure expression produces a closure value with a unique, anonymous
    // type that cannot be written out."
    //
    // source: https://doc.rust-lang.org/beta/reference/types/closure.html
    //

    // let's use this little helper function to get the types of some variables
    fn print_type_of<T>(label: &str, _: &T) {
        println!("{}: {}", label, std::any::type_name::<T>())
    }

    // type is `closures::main::{{closure}}`, an anonymous type
    let closure = || println!("Hello, World!");
    print_type_of("example 1", &closure);

    // You can coerce a closure to a function pointer, changing its type
    let closure: fn() -> () = || println!("Hello, World!");
    print_type_of("example 2", &closure);

    // closures can only be coerced to `fn` types if they do not capture any variables
    let x = 42;
    let closure = || x; // You cannot specify an explicit type here. ("...type that cannot be written out")
    print_type_of("example 3", &closure);

    //
    // Point 2
    //
    // Parameter and return types of function pointers
    //

    // The return type is () by default, so -> () can be dropped here
    let closure: fn() = || println!("Hello, World!");
    print_type_of("example 4", &closure);

    // -> i32 must be specified here, since 42 is the returned value
    let closure: fn() -> i32 = || 42;
    print_type_of("example 5", &closure);

    // Similarly, parameter types must be specified
    let closure: fn(i32, &f32, &mut String) = |x, y, z| println!("received {}, {}, {}", x, y, z);
    print_type_of("example 6", &closure);

    //
    // Point 3
    //
    // `where` clauses are just a more expressive way of writing type bounds.
    //

    fn call_closure_A<F>(f: F) -> i32
        where
            F: FnOnce() -> i32 {
        f()
    }

    fn call_closure_B<F: FnOnce() -> i32>(f: F) -> i32 {
        f()
    }

    // Note that `|| 42` is an Fn closure (captures no values from its environment)
    // but it can be passed to a function which expects an FnOnce closure, because
    // the FnOnce trait extends the Fn trait.

    // In general, write your functions to accept FnOnce (or FnMut) to make them
    // more flexible for the caller, unless you have a specific reason to do otherwise.

    call_closure_A(|| 42);
    call_closure_B(|| 42);

    //
    // Point 4
    //
    // FnOnce closures can only be called once.
    //

    fn call_closure<F: FnOnce()>(f: F) {
        f()
    }

    let x = Box::new(42);

    // This is an FnOnce closure because it takes ownership of a captured value.
    // Obviously, it can only be called one time, because x can only be dropped once.
    let closure = || drop(x);

    call_closure(closure);

    // Uncomment the line below to see a compilation error
    // "closure cannot be moved more than once ... moving the variable `x` out of its environment"
    // call_closure(closure);

    //
    // Point 5
    //
    // Closures get their "flavours" (Fn, FnMut, FnOnce) from _the values they
    // close over_, not the arguments they take in their signatures.
    //

    fn call_Fn_closure<F: Fn(Box<i32>)>(f: F) {
        f(Box::new(0))
    }

    // This closure is Fn (it can be called multiple times), even though it
    // consumes the argument passed to it.
    let closure = |x| drop(x);

    call_Fn_closure(closure);
    call_Fn_closure(closure);
    call_Fn_closure(closure);

    // This makes sense, as each time closure is called, new arguments must be
    // passed to it. Those new arguments are consumed each time closure is called.

    //
    // Point 6
    //
    // Bringing it all together.
    //

    // So, we have
    // - an Fn, FnMut, or FnOnce closure, based on values captured from the environment
    // - parameter types passed into the closure
    // - the return type of the closure
    //
    // All of the above give the full type of the closure argument

    fn uses_complex_closure<F>(f: F) -> char
        where
        F: FnOnce(Box<i32>, &f64, &mut String) -> bool {

        let x = Box::new(0);

        if f(Box::new(42), &3.14, &mut String::from("hello")) {
            println!("Wahoo!");
            'W'
        } else {
            println!("Not wahoo.");
            'L'
        }
    }

    let a = Box::new(-1);

    let closure = |x, y: &f64, z: &mut String| {
        drop(a); // takes ownership of this value from the environment
        drop(x); // takes ownership of x

        println!("{}", y); // only needs an immutable reference to y

        z.clear(); // needs a mutable reference to z

        true // returns bool
    };

    uses_complex_closure(closure);
    // uses_complex_closure(closure); // uncomment to see a compilation error

    // Above:
    //
    // the function `uses_complex_closure`
    // - takes a single argument of type F
    // - returns a value of type char
    //
    // the argument `f` is a closure which
    // - is FnOnce, it captures some value from its environment and so can only be invoked one time
    // - takes three arguments: an owned value, a reference, and a mutable reference
    // - returns a boolean

    // More complexity ahead...
    // ...this doesn't even touch on lifetimes!

}
