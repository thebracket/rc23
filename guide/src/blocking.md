# Async Blocking

You can sometimes get around a function taking too long---and stalling the task pool---with `yeild_now`. Sometimes, you *really* want to block.

Let's look at a simple example:

> The code is in the `blocking` project.

```rust
fn print_sync(id: i32) -> i32 {
    for i in 0 .. 10 {
        println!("{id}:{i}");
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    }
    1
}

#[tokio::main]
async fn main() {
    let result = tokio::task::spawn_blocking(|| print_sync(0)).await;
    println!("{result:?}");
}
```

> Note that if you don't `await`, the blocking task will run in the background until it is done. You can also keep the `JoinHandle` for later.

Blocking Tasks in Tokio *are* running in a thread - but that thread is wrapped inside the async system. This is one way to have your cake and eat it too: you can block, but you can also use async features.

> Tokio retains a large thread pool, so you can spawn many blocking tasks without worrying about running out of threads.