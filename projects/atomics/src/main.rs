use std::thread;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

fn main() {
    const NUM_THREADS: usize = 32;
    let numbers = (0..10000).collect::<Vec<u32>>();
    static SUM: AtomicU32 = AtomicU32::new(0);
    thread::scope(|scope| {
        for chunk in numbers.chunks(NUM_THREADS) {
            scope.spawn(move || {
                chunk.iter().for_each(|n| {
                    SUM.fetch_add(*n, Ordering::Relaxed);
                });
            });
        }
    });
    println!("TOTAL: {}", SUM.load(Ordering::Relaxed));
}
