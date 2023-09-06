use std::thread;

fn main() {
    const NUM_THREADS: usize = 32;
    let numbers = (0..10000).collect::<Vec<u32>>();
    static mut sum: u32 = 0;
    thread::scope(|scope| {
        for chunk in numbers.chunks(NUM_THREADS) {
            scope.spawn(move || {
                chunk.iter().for_each(|n| {
                        sum += *n;
                });
            });
        }
    });
    println!("TOTAL: {sum}");
}
