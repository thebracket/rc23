use tokio::task::yield_now;

async fn print(id: i32) {
    for i in 0 .. 10 {
        println!("{id}:{i}");
        yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    println!("Awaiting");
    print(0).await;
    
    println!("Joining - waiting for all tasks");
    tokio::join!(
        print(0), print(1), print(2)
    );
}
