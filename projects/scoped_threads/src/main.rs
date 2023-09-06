use std::thread::{self, ScopedJoinHandle};

fn main() {
    const NUM_THREADS: usize = 4;
    let numbers = (0 .. 5000).collect::<Vec<u32>>();
    let chunks = numbers.chunks(NUM_THREADS);
    let sum: u32 = thread::scope(|scope| {
        let mut handles: Vec<ScopedJoinHandle<u32>> = Vec::new();
        for chunk in chunks {
            handles.push(scope.spawn(move || {
                chunk.iter().sum()
            }));
        }
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    });
    
    println!("{sum}");
}