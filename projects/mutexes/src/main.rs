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
