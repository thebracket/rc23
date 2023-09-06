# TCP Server & Client

Let's put what we've learned together and build a highly performant TCP server. You can use this pattern over and over---I do!

> The source code for this is in the `tcpserver` example, located in the `projects` directory of the repository.

```rust
use std::{time::Duration, sync::{Arc, atomic::AtomicUsize}};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    net::TcpStream,
    spawn,
    select,
    time::sleep
};
use anyhow::Result;

const BIND: &str = "127.0.0.1:8123";

async fn server_loop(mut quit: tokio::sync::broadcast::Receiver<()>) -> Result<()> {
    println!("Starting Server");
    let counter = Arc::new(AtomicUsize::new(0));
    let listener = TcpListener::bind(BIND).await?;
    loop {
        let counter = counter.clone();
        select! {
            _ = quit.recv() => break,
            Ok((socket, _)) = listener.accept() => {
                tokio::spawn(async move {
                    handle_connection(socket, counter).await
                });
            }
        }
    }

    println!("Quitting Server");
    println!("The server handled {} bytes of data", counter.load(std::sync::atomic::Ordering::Relaxed));
    Ok(())
}

async fn handle_connection(mut socket: TcpStream, counter: Arc<AtomicUsize>) -> Result<()> {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = socket.read(&mut buffer).await?;

        // No bytes read - disconnect
        if bytes_read == 0 {
            break;
        }

        // Send the data back
        socket.write_all(&buffer[..bytes_read]).await?;
        counter.fetch_add(bytes_read, std::sync::atomic::Ordering::Relaxed);
    }
    Ok(())
}

async fn tcp_client() -> Result<()> {
    let mut socket = TcpStream::connect(BIND).await?;
    let mut buf = [0; 1024];
    for _ in 0..100 {
        let _ = socket.write_all(b"Hello World!").await;
        let bytes_read = socket.read(&mut buf).await?;
        if bytes_read == 0 {
            println!("Client disconnected");
            break;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create a broadcast channel to end the program
    let (tx, _rx) = tokio::sync::broadcast::channel::<()>(1);

    // Start the TCP server
    spawn(server_loop(tx.subscribe()));

    // Short pause to let the playground catch up
    sleep(Duration::from_millis(100)).await;

    // Spawn several TCP clients
    for _ in 0..3 {
        spawn(tcp_client());
    }

    // Sleep
    sleep(Duration::from_secs(1)).await;

    // Send quit
    tx.send(()).unwrap();

    // Short pause to let the playground catch up
    sleep(Duration::from_millis(100)).await;
    Ok(())
}

```