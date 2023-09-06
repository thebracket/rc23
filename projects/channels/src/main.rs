use std::thread;
use std::sync::mpsc;

fn main() {
    const NUM_THREADS: i32 = 4;
    let (tx, rx) = mpsc::channel::<i32>();

    thread::scope(|scope| {
        scope.spawn(move || {
            for _ in 0..NUM_THREADS {
                let n = rx.recv().unwrap();
                println!("{n}");
            }
        });
    
        for i in 0..NUM_THREADS {
            let my_tx = tx.clone();
            scope.spawn(move || {
                my_tx.send(i).unwrap();
            });
        }
    });
}
