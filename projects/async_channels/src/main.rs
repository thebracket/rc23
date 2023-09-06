use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    const NUM_TASKS: i32 = 4;
    let (tx, mut rx) = mpsc::channel::<i32>(10); // Tokio channels must have a max size

    for i in 0..NUM_TASKS {
        let my_tx = tx.clone();
        tokio::spawn(async move {
            my_tx.send(i).await.unwrap();
        });
    }

    for _ in 0..NUM_TASKS {
        let received = rx.recv().await.unwrap();
        println!("{received}");
    }
}
