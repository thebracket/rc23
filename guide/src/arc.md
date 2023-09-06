# Arc and Data Destruction

Sometimes, it's not obvious where ownership of a variable lies. This is *especially* true for shared resources.

`Arc` works by *reference counting* a variable. Every time you `clone` the variable---the reference count is increased, but you're still looking at the *same* variable. When a variable leaves scope, the reference count is decremented. When the reference count hits zero, the variable is destroyed.

`Arc` uses an atomic number for sharing the reference count---so it's safe to use across threads. (The single-threaded variant, `Rc` uses a `usize`---it's slightly faster, but isn't safe to use with threads)

> This is the basis of some garbage collection algorithms! Rust *does* have some opt-in garbage collection, after all!

Let's create an example:

> The code is in the `arcs` project.

```rust
use std::thread;
use std::sync::Arc;

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
    let my_dog = Arc::new(MyPet { name: "Henry".to_string() });

    thread::scope(|scope| {
        for _ in 0..NUM_THREADS {
            let cloned_henry = my_dog.clone();
            scope.spawn(move || {
                println!("{cloned_henry:?}")
            });
        }
    });
}
```

The first thing to note is that you are cloning Henry for every thread. This *doesn't* make a copy of Henry---it just obtains a shared pointer to Henry. The second thing to note is that Henry is just destroyed once: your resource isn't duplicated, it's just shared.