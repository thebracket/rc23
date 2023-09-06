use tokio::sync::mpsc;
use tokio::sync::broadcast;
use tokio::select;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let (send_quit, _recv_quit) = broadcast::channel(8);
    let (send_print, mut recv_print) = mpsc::channel(8);

    for i in 0..4 {
        let my_sender = send_print.clone();
        let mut my_quitter = send_quit.subscribe();
        tokio::spawn(async move {
            loop {
                select! {
                    _ = sleep(Duration::from_secs_f32(0.1)) => my_sender.send(i).await.unwrap(),
                    _ = my_quitter.recv() => break,
                }
            }
            println!("Quitting task {i}");
        });
    }
    
    for _ in 0 .. 10 {
        let n = recv_print.recv().await.unwrap();
        println!("{n}");
    }
    send_quit.send(0).unwrap();
    sleep(Duration::from_secs(1)).await;
}
