use std::thread::{self, JoinHandle};

fn main() {
    const NUM_THREADS: usize = 4;
    let numbers = (0 .. 5000).collect::<Vec<u32>>();
    let chunks = numbers.chunks(NUM_THREADS);
    let mut handles: Vec<JoinHandle<u32>> = Vec::new();
    for chunk in chunks {
        handles.push(thread::spawn(move || {
            chunk.iter().sum()
        }));
    }
    
    let sum: u32 = handles.into_iter().map(|th| th.join().unwrap()).sum();
    println!("{sum}");
}