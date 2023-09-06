# Safety with Atomics

An *atomic* is a special type that is synchronized across threads. It's a CPU primitive on most platforms, so it's also very fast. The bad news is that there aren't atomic versions of every type.

Let's take the unsafe example from a moment ago, and make it safe:

> The code is in the `atomics` project.

```rust
use std::thread;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

fn main() {
    const NUM_THREADS: usize = 32;
    let numbers = (0..10000).collect::<Vec<u32>>();
    static SUM: AtomicU32 = AtomicU32::new(0);
    thread::scope(|scope| {
        for chunk in numbers.chunks(NUM_THREADS) {
            scope.spawn(move || {
                chunk.iter().for_each(|n| {
                    SUM.fetch_add(*n, Ordering::Relaxed);
                });
            });
        }
    });
    println!("TOTAL: {}", SUM.load(Ordering::Relaxed));
}
```

You get the same result every time, no need for `unsafe`---and it's still fast!

Notice that you are specifying an `Ordering`. There's a few different choices; `Relaxed` doesn't care what *order* the operations occur in---just that they occur, and don't overwrite one another. There are much stricter options available if you need them.