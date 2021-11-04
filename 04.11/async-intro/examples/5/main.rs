use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

enum Command {
    Get(String, oneshot::Sender<Bytes>),
}

async fn send(tx: tokio::sync::mpsc::Sender<Command>, s: &'static str) {
    let (resp_tx, resp_rx) = oneshot::channel();
    tokio::spawn(async move {
        let cmd = Command::Get(s.to_string(), resp_tx);

        let _ = tx.send(cmd).await;

        println!("Got value from the server; result={:?}", resp_rx.await)
    });
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(4);
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:1729").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get(cmd, resp) => {
                    let result = client.get(&cmd).await.unwrap().unwrap();
                    let _ = resp.send(result);
                }
            }
        }
    });

    send(tx.clone(), "message 0").await;
    send(tx.clone(), "message 1").await;

    manager.await.unwrap();
}
