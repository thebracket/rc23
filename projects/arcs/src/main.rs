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
