use std::thread;

fn main() {
    const NUM_THREADS: usize = 32;
    let numbers = (0..1000000).collect::<Vec<u64>>();
    static mut SUM: u64 = 0;
    thread::scope(|scope| {
        for chunk in numbers.chunks(NUM_THREADS) {
            scope.spawn(move || {
                for _ in 0 .. 100 {
                    chunk.iter().for_each(|n| {
                        unsafe {
                            SUM += *n;
                        }
                    });
                }
            });
        }
    });
    unsafe {
        println!("TOTAL: {SUM}");
    }
}