use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct F {}

impl Future for F {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
        println!("Polled.");

        // Poll::Ready(())
        Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    let h = tokio::spawn(F {});

    let _ = h.await;
}
