# Chapter 20: Std misc

The `std` library provides many additional types.

If you don't need them, you can use [`no_std`](https://docs.rust-embedded.org/book/intro/no-std.html) to forgo them and reduce the size of your application binary.

## 20.1 Threads

`std::thread::spawn()` can be used to spawn OS threads.

> "the argument of this function is a moving closure"

...it doesn't need to be. The following would also be valid

```rs
// --snip--
        children.push(thread::spawn(|| {
            println!("this is thread number");
// --snip--
```

### 20.1.1 Testcase: map-reduce

(Walk through this example.)

## 20.2 Channels

`mpsc` stands for "multiple producer, single consumer" -- we can clone the transmitter, but all transmitters will send to the same receiver.

The example in this section and in the previous section are good ones to bookmark for when you start to work with multiple threads in Rust.

This example doesn't do any "useful" work, it just creates data and sends it to a receiver via multiple threads. But we can see at the end that the data arrives "out of order".

`tokio` async tasks, channels

## 20.3 Path

Aside: here's an article about [the complexity of the `Path` API in Rust vs. Go](https://fasterthanli.me/articles/i-want-off-mr-golangs-wild-ride#simple-is-a-lie). Rust exposes this complexity, while Go attempts to hide it, sometimes failing in unexpected and nonobvious ways.

The `OsString` type -- [why do we need it?](https://doc.rust-lang.org/std/ffi/struct.OsString.html)

Note there is also `CString` [for compatibility with C](https://doc.rust-lang.org/std/ffi/struct.CString.html).

You'll probably create `Path`s with `Path::new()` most of the time.

## 20.4 File I/O

> "...all the File methods return the io::Result<T> type, which is an alias for Result<T, io::Error>."

...this is not strictly true -- [see the docs](https://doc.rust-lang.org/std/fs/struct.File.html). For example, `bytes()` returns `Bytes<Self>`.

But yes, "failure paths" are explicit -- you must explicitly handle `Err` in Rust just like how you must handle `Failure` in Scala.

### 20.4.1 open

> "...open a file in read-only mode."

...but what about writing to a file? Check out [`std::fs::OpenOptions`](https://doc.rust-lang.org/std/fs/struct.OpenOptions.html).

### 20.4.2 create

`create` opens a file and _deletes any preexisting contents_.

Use `create_new` to fail if the file already exists.

Use `OpenOptions` to set `append` to `true` if you want to append to the existing contents.

### 20.4.3 read_lines

Use a buffered reader instead of reading the entire file into memory.

## 20.5 Child processes

Use `std::process::Command` to execute a shell command from within a Rust program.

Use `.output()` to capture [the STDOUT and STDERR output](https://doc.rust-lang.org/std/process/struct.Command.html#method.output) of that command.

### 20.5.1 Pipes

> "The `std::Child` struct represents a running child process, and exposes the `stdin`, `stdout` and `stderr` handles for interaction with the underlying process via pipes."

In case you want to send input to other command-line commands ("pretending" to be STDIN).

### 20.5.2 Wait

Capture the exit status of a command-line command with `std::process::Child::wait()`.

## 20.6 Filesystem Operations

(Walk through this example.)

Note the line...

```rs
if cfg!(target_family = "unix") {
```

...this will only execute if the compilation target for the binary is *NIX.

## 20.7 Program arguments

Available through `std::env::args`, but I highly recommend [`clap`](https://docs.rs/clap/latest/clap/#example).

### 20.7.1 Argument parsing

If you don't need all of the features of `clap`, it can be more performant to parse args directly.

## 20.8 Foreign Function Interface

Also known as FFI, this is how Rust communicates with C.

Unless you're going to do C-interop programming, you don't really need to know how this works. But good to know that it's there.