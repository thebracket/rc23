# Async Channels and Select

Tokio mirrors much of the standard library with async versions. There are async versions of channels that work a lot like threaded channels.

> The code is in the `async_channels` project.

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    const NUM_TASKS: i32 = 4;
    let (tx, mut rx) = mpsc::channel::<i32>(10); // Tokio channels must have a max size

    for i in 0..NUM_TASKS {
        let my_tx = tx.clone();
        tokio::spawn(async move {
            my_tx.send(i).await.unwrap();
        });
    }

    for _ in 0..NUM_TASKS {
        let received = rx.recv().await.unwrap();
        println!("{received}");
    }
}
```

Tokio also adds a `select!` macro that waits for multiple futures at once---and acts on the first one to reply. This is extremely useful when you want to wait for more than one input source. Let's make use of another Tokio channel type---`broadcast`---to implement tasks that continually send data to a channel and stop when they receive a stop signal.

> The code is in the `async_select` project.

```rust
use tokio::sync::mpsc;
use tokio::sync::broadcast;
use tokio::select;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let (send_quit, _recv_quit) = broadcast::channel(8);
    let (send_print, mut recv_print) = mpsc::channel(8);

    for i in 0..4 {
        let my_sender = send_print.clone();
        let mut my_quitter = send_quit.subscribe();
        tokio::spawn(async move {
            loop {
                select! {
                    _ = sleep(Duration::from_secs_f32(0.1)) => my_sender.send(i).await.unwrap(),
                    _ = my_quitter.recv() => break,
                }
            }
            println!("Quitting task {i}");
        });
    }
    
    for _ in 0 .. 10 {
        let n = recv_print.recv().await.unwrap();
        println!("{n}");
    }
    send_quit.send(0).unwrap();
    sleep(Duration::from_secs(1)).await;
}
```
