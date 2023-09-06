# Mixing Threads and Async

You already saw that you can use `spawn_blocking` to spawn threads inside the Tokio runtime. There are other options available.

## Calling into the Runtime From Outside

You can get the current Tokio runtime "handle" with `Handle::current()`. This gives you options to call *into* an existing runtime from a synchronous context---without needing to create a whole new runtime.

```rust
use tokio::runtime::Handle;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
 
#[tokio::main]
async fn main() {
    let runtime_handle = Handle::current();
    let (tx, mut rx) = mpsc::channel(10);

    thread::spawn(move || {
        let mut i = 0;
        loop {
            i += 1;
            let tx = tx.clone();
            runtime_handle.spawn(async move {
                tx.send(i).await.expect("Channel Closed (tx)");
            });
            thread::sleep(Duration::from_secs_f32(0.2));
        }
    });
    
    for _ in 0 .. 20 {
        let n = rx.recv().await.expect("Channel Closed (rx)");
        println!("{n}");
    }
}
```

> All the threaded systems work, too. You can make threaded channels and have async all into a threaded system---and use handles to call back. You can even have multiple runtimes and use their handles to talk to one another.