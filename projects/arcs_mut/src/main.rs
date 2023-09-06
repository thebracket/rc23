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
