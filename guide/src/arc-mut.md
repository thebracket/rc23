# Mutable Arcs

The `Arc` type implements a trait called `Send`: it is safe to *send* it between threads (as we did with Henry!). `Arc` is not `Sync`: it is not safe to *share* it between threads. `Mutex` and the atomic types implement `Sync`---so they are safe to share between threads.

> You *cannot* alter data inside an `Arc` without also providing a synchronization primitive.

So let's extend the previous example to protect Henry with a `Mutex`:

> The code is in `arcs_mut`

```rust
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
struct MyPet {
    name: String
}

impl Drop for MyPet {
    fn drop(&mut self) {
        println!("{} was destroyed", self.name)
    }
}

fn main() {
    const NUM_THREADS: usize = 4;
    let my_dog = Arc::new(Mutex::new(MyPet { name: "Henry".to_string() }));

    thread::scope(|scope| {
        for _ in 0..NUM_THREADS {
            let cloned_henry = my_dog.clone();
            scope.spawn(move || {
                let mut lock = cloned_henry.lock().unwrap();
                lock.name += "!";
            });
        }
    });

    let lock = my_dog.lock().unwrap();
    println!("{}", lock.name);
}
```

As expected: the shared instance of Henry had four `!` appended to his name. It starts to get unwieldy when you are combining `Arc<Mutex<MyType>>`. It can also get inefficient. If you had multiple threads that each need to work on *part* of the shared data at the same time---they can't, because the `Mutex` will serialize their access.

You can get around this with *interior mutability*.

> The code is in `arcs_interior_mut`

```rust
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
struct MyPet {
    name: Mutex<String>
}

impl Drop for MyPet {
    fn drop(&mut self) {
        let lock = self.name.lock().unwrap();
        println!("{} was destroyed", *lock)
    }
}

fn main() {
    const NUM_THREADS: usize = 4;
    let my_dog = Arc::new(
        MyPet { name: Mutex::new("Henry".to_string()) }
    );

    thread::scope(|scope| {
        for _ in 0..NUM_THREADS {
            let cloned_henry = my_dog.clone();
            scope.spawn(move || {
                let mut lock = cloned_henry.name.lock().unwrap();
                *lock += "!";
            });
        }
    });

    let lock = my_dog.name.lock().unwrap();
    println!("{}", *lock);
}
```

Add a constructor to hide the Mutex creation, and you have a much clearer interface---and you can have multiple threads access different parts of the shared data concurrently.