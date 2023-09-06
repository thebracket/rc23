use tokio::runtime::Handle;
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;
 
#[tokio::main]
async fn main() {
    let runtime_handle = Handle::current();
    let (tx, mut rx) = mpsc::channel(10);

    thread::spawn(move || {
        let mut i = 0;
        loop {
            i += 1;
            let tx = tx.clone();
            runtime_handle.spawn(async move {
                tx.send(i).await.expect("Channel Closed (tx)");
            });
            thread::sleep(Duration::from_secs_f32(0.2));
        }
    });
    
    for _ in 0 .. 20 {
        let n = rx.recv().await.expect("Channel Closed (rx)");
        println!("{n}");
    }
}
