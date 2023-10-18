use tokio::task;

async fn print(id: i32) {
    for i in 0 .. 10 {
        println!("{id}:{i}");
        task::yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(print(0));
    tokio::spawn(print(1));
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
