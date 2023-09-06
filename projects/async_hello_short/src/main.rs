async fn hello() {
    println!("Hello, World!");
}

#[tokio::main]
async fn main() {
    hello().await;
}