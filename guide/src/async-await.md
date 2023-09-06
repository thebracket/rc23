# Different Waiting Strategies

Whenever a task *awaits* something, it hands over control to the runtime. The runtime will make sure that it wakes up again when the thing it's waiting for is ready. There are a few different ways to do this, and they all have different tradeoffs.

Let's put together a little framework to look at the different ways in which you can wait for things or yield control.

The two most basic systems are `await` and `join`. Awaiting waits for a single future to be ready. Join waits for multiple futures to be ready, finishing when they all complete. The `yield_now` function is different: it doesn't *wait* for anything, but it moves the current task to the back of the task queue. If other tasks can run, they will---otherwise, execution will continue.

> If you have a large task that doesn't wait for anything, you can use `yield_now` to make sure that other tasks get a chance to run.

> The code is in the `yielding` project.

```rust
use tokio::task::yield_now;

async fn print(id: i32) {
    for i in 0 .. 10 {
        println!("{id}:{i}");
        yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    println!("Awaiting");
    print(0).await;
    
    println!("Joining - waiting for all tasks");
    tokio::join!(
        print(0), print(1), print(2)
    );
}
```

> We'll talk about `select` in the next section.

## Spawning

Tokio's `spawn` submits a future for execution, and doesn't require that you wait for it. Spawned tasks are encouraged---but not required---to start on a different thread if you are in a multi-threaded runtime environment.

> The code is in the `spawning` project.

```rust
use tokio::task::yield_now;

async fn print(id: i32) {
    for i in 0 .. 10 {
        println!("{id}:{i}");
        yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(print(0));
    tokio::spawn(print(1));
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
```
