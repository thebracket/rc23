use std::thread;

fn main() {
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(|| {
            println!("Hello from thread {:?}", thread::current().id());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
