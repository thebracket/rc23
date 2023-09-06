use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from thread {:?}", thread::current().id());
    });
    println!("Hello from the main thread");
    handle.join().unwrap();
}
