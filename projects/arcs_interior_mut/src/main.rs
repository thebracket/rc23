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
