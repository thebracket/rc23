# Let's Make A Data Race

> Don't do this in real code! This is illustrating the problem, and how Rust helps you.

Suppose we want to sum a vector of numbers again---we'll pretend its some real work. Instead of summing the results of each chunk, we'll increment a shared counter.

Let's try it the non-Rusty way:

> The code is in the `data_race` project. The local version is slightly different, because the playground runs much more slowly than my laptop.

```rust
use std::thread;

fn main() {
    const NUM_THREADS: usize = 32;
    let numbers = (0..10000).collect::<Vec<u64>>();
    static mut SUM: u64 = 0;
    thread::scope(|scope| {
        for chunk in numbers.chunks(NUM_THREADS) {
            scope.spawn(move || {
                chunk.iter().for_each(|n| {
                    unsafe {
                        SUM += *n;
                    }
                });
            });
        }
    });
    unsafe {
        println!("TOTAL: {SUM}");
    }
}
```

Run it a few times, and you get different answers! You've got a *data-race*. Incrementing an integer isn't just one operation:

1. The CPU has to load the current value into memory (or a register).
2. The CPU adds to the value.
3. The CPU stores the result back into memory.

Each thread may or may not interrupt another thread between any of these steps. If you interrupt a thread between steps 1 and 2, you'll lose the result of the other thread's work.

## Good news: Rust has your back!

Notice that the above code won't compile at all without the use of the `unsafe` keyword. Rust's safety system knows that there's a problem, and won't let you do it unless you explicitly remove your seatbelt.

This won't compile:

> The code is in `data_race_averted`.

```rust
use std::thread;

fn main() {
    const NUM_THREADS: usize = 32;
    let numbers = (0..10000).collect::<Vec<u32>>();
    static mut sum: u32 = 0;
    thread::scope(|scope| {
        for chunk in numbers.chunks(NUM_THREADS) {
            scope.spawn(move || {
                chunk.iter().for_each(|n| {
                        sum += *n;
                });
            });
        }
    });
    println!("TOTAL: {sum}");
}
```