# Basic Async

So far, we've talked about *threads*. Threads are ideal for CPU-bound workloads: you write normal Rust, divide your workload, and each thread powers through the tasks at hand.

Where threads aren't so great is for heavily multi-user setups, like servers. If you have a server that needs to handle 1000 users, you don't want to have 1000 threads. That's a lot of overhead, and a lot of wasted resources. If you have 50,000 users---most operating systems aren't going to let you make that many threads.

Async is a bit different, both in programming and functionality. Async is all about *tasks*. Tasks are cooperatively multi-tasked: while a task is running, it won't be interrupted by another task on the same thread. That means that async *isn't* great for CPU-bound tasks. Instead, it's best for tasks that do a lot of waiting. Most of the time, a server is waiting for data---from a user, a database, a REST request, etc. That's where async shines: every time a task `awaits` something else, another task gets to go. You're effectively benefiting from latency, maximizing throughput.

## Async/Await

Async is different from threads in that you can't just add `async` to a function and expect to be able to await it. You have to run async inside a *runtime*. Tokio is a popular runtime, and it's what we'll be using here.

Here's "Hello Tokio", the long version:

> The code is in `async_hello`

```rust
use tokio::runtime::Runtime;

async fn hello() {
    println!("Hello, World!");
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        hello().await;
    });
}
```

Every async runtime has an equivalent to `block_on`---you hand control to the runtime, and it executes async functions inside. Tokio lets you specify single or multi-threaded, and a bunch of other options. We'll be using the multi-threaded version.

Here's the short version:

> The code is in `async_hello_short`

```rust
async fn hello() {
    println!("Hello, World!");
}

#[tokio::main]
async fn main() {
    hello().await;
}
```

> You might need to only have part of your program async, so it's important to remember the long version exists!

## So What's Going on Here?

Any function that is decorated with `async` returns a `Future`. Futures don't run when you call them. Calling `hello()` on its own doesn't do anything. Instead, you have to `await` the future. When you `await` a future, you're telling the runtime to run the future until it's done.