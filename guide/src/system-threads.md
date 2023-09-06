# Creating System Threads

> A "system thread" is an Operating System managed thread. System threads are relatively "heavy"---they have a full stack, execution context, and are (relatively) slow to create. Most Operating Systems won't let you have thousands of threads at one time.

Rust's standard library provides a `thread` module that allows you to create system threads. Let's make the "hello wold" of threads:

> This example is located in the `hello_threads` project folder.

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from thread {:?}", thread::current().id());
    });
    println!("Hello from the main thread");
    handle.join().unwrap();
}
```

> Note that spawning the thread creates a `handle`. Join handles allow you to wait for a thread to finish. When your program terminates, all threads stop with it---so you need to wait for threads to finish before your program exits.

Let's create 10 threads, and see what happens:

> This example is located in the `ten_threads` project folder.

```rust
use std::thread;

fn main() {
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(|| {
            println!("Hello from thread {:?}", thread::current().id());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

System threads with `std::thread` are the most basic threading primitive in Rust---they are a thin wrapper over the operating system's thread provisioning.