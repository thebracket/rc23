# Scoped Threads

Rust 1.63 added a new feature, *Scoped Threads*. Scoped threads are a new threading primitive that allows you to create threads that are guaranteed to terminate before the end of the current scope.

This is more useful than it sounds:
* It fits well with "fan out" CPU-bound workloads.
* Knowing that threads will terminate makes it easier to access locally scoped data.

Let's use system threads to compute the sum of a vector of numbers. This seems reasonable enough:

> This code is in the `scope_fail` project.

```rust
use std::thread::{spawn, JoinHandle};

fn main() {
    const NUM_THREADS: usize = 4;
    let numbers = (0 .. 5000).collect::<Vec<u32>>();
    let chunks = numbers.chunks(NUM_THREADS);
    let mut handles: Vec<JoinHandle<u32>> = Vec::new();
    for chunk in chunks {
        handles.push(spawn(move || {
            chunk.iter().sum()
        }));
    }
    
    let sum: u32 = handles.into_iter().map(|th| th.join().unwrap()).sum();
    println!("{sum}");
}
```

But it doesn't compile! `numbers` doesn't live long enough. You *could* solve this by making a `static` variable. Instead, let's solve this by using scoped threads:

> This is in the `scoped_threads` project.

```rust
use std::thread::{self, ScopedJoinHandle};

fn main() {
    const NUM_THREADS: usize = 4;
    let numbers = (0 .. 5000).collect::<Vec<u32>>();
    let chunks = numbers.chunks(NUM_THREADS);
    let sum: u32 = thread::scope(|scope| {
        let mut handles: Vec<ScopedJoinHandle<u32>> = Vec::new();
        for chunk in chunks {
            handles.push(scope.spawn(move || {
                chunk.iter().sum()
            }));
        }
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    });
    
    println!("{sum}");
}
```