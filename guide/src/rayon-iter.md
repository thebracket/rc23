# Rayon: Easy Mode

Rayon is a Rust library similar to Intel's Threaded Building Blocks for C++. It provides a scoped and pooled threading API, and also provides parallel iterators. For many CPU-bound operations, you can simplify the entire process to a single iterator chain.

Let's rewrite our summing system to use Rayon:

> The code is in the `easy_rayon` project.

```rust
use rayon::prelude::*;

fn main() {
    let numbers = (0 .. 5000).collect::<Vec<u32>>();
    let sum: u32 = numbers
        .par_iter()
        .sum();
    println!("{sum}");
}
```

Rayon's scoped threads operate similarly to Rust Scoped Threads, but they operate on a global thread pool by default (which you can configure).