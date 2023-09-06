fn print_sync(id: i32) -> i32 {
    for i in 0 .. 10 {
        println!("{id}:{i}");
        std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
    }
    1
}

#[tokio::main]
async fn main() {
    let result = tokio::task::spawn_blocking(|| print_sync(0)).await;
    println!("{result:?}");
}
