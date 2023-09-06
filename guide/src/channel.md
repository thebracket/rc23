# Sending Data in Channels

The standard library includes a Multi-Producer, Single Consumer channel type. It's very useful for sending data between threads. Any number of threads can *send* data into it, but they can only be received by one thread. (`Crossbeam` provides a Multi-Producer, Multi-Consumer channel type, if you need it).

Let's look at a simple example:

> The code is in `channels`

```rust
use std::thread;
use std::sync::mpsc;

fn main() {
    const NUM_THREADS: i32 = 4;
    let (tx, rx) = mpsc::channel::<i32>();

    thread::scope(|scope| {
        scope.spawn(move || {
            for _ in 0..NUM_THREADS {
                let n = rx.recv().unwrap();
                println!("{n}");
            }
        });
    
        for i in 0..NUM_THREADS {
            let my_tx = tx.clone();
            scope.spawn(move || {
                my_tx.send(i).unwrap();
            });
        }
    });
}
```

You don't have to send `i32`---you can send any type that implements `Send` (which is most types). I'm fond of sending:

* Enumerations (with or without data inside) to issue commands.
* Function pointers, allowing me to send a function to another thread to execute.
* `Arc` types---often with mutability inside. Recipients can do the work they need, and when no thread still retains a reference to the `Arc`, it will be destroyed.