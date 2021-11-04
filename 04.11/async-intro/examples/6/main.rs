use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

async fn my_async_fn() {
    println!("hello from async");
    let _socket = TcpStream::connect("127.0.0.1:1729").await.unwrap();
    println!("async TCP operation complete");
}

#[tokio::main]
async fn main() {
    let what_is_this = my_async_fn();

    sleep(Duration::from_secs(1)).await;

    what_is_this.await;
}
