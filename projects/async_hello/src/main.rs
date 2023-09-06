use tokio::runtime::Runtime;

async fn hello() {
    println!("Hello, World!");
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        hello().await;
    });
}
