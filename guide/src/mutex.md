# Mutexes

If you need to share a type that isn't a primitive---or a complex type---you can safely wrap it in a Mutex. A Mutex is a *mutual exclusion* system; only one entity can have it "locked" at the same time. If another entity tries to lock it, it will wait until the first entity is done.

> Warning: This is a great way to un-thread your program. If you have a lot of threads waiting on a Mutex, you're not getting any benefit from the threads. You're better off with a single thread.

Let's look at a simple mutex example:

```rust
use std::sync::Mutex;
use std::thread;

fn main() {
    const NUM_THREADS: usize = 8;
    static BIGSTRING: Mutex<String> = Mutex::new(String::new());

    thread::scope(|scope| {
        for thread_num in 0..NUM_THREADS {
            scope.spawn(move || {
                for i in 0..5 {
                    let mut lock = BIGSTRING.lock().unwrap();
                    *lock += &format!("{thread_num}:{i}.");
                }
            });
        }
    });
    let lock = BIGSTRING.lock().unwrap();
    println!("{}", *lock);
}
```

Note how we've wrapped our shared `String` in a `Mutex`. Then when it comes time to access it, we call `lock()` on it. This returns a `MutexGuard` which is a smart pointer that will automatically unlock the mutex when it goes out of scope. We can then dereference it to get the value inside.

> Note: You CAN deadlock, and Rust WON'T help you! Locking an already locked mutex will make your program freeze. Be a little more fearful here!

> You can also use `try_lock()` which will return an error if the mutex is already locked. This is useful if you want to do something else if the mutex is locked.

There's also `RwLock` which is like a mutex---but you can lock with `read()` or `write()`. A read-lock can only read the contents, a write lock can make changes. You can have as many read locks as you want---but only one write lock at a time (and write locks don't start until all read locks are done).