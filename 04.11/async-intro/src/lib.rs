use futures::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Sender<T> {
    marker: PhantomData<T>,
}

pub struct Receiver<T> {
    marker: PhantomData<T>,
}

pub fn chan<T: Send>() -> (Sender<T>, Receiver<T>) {
    let marker = PhantomData {};
    (Sender { marker }, Receiver { marker })
}

impl<T: Send> Sender<T> {
    pub async fn send(self, val: T) {
        Messanger { _val: val }.await
    }
}

struct Messanger<T> {
    _val: T,
}

impl<T> Future for Messanger<T> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

impl<T> Future for Receiver<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio::time::{sleep, Duration};

    #[derive(PartialEq, Eq, Debug)]
    enum Ordering<T> {
        Sending(T),
        Receiving,
        Sent,
        Received(T),
    }

    #[tokio::test]
    async fn basic_send() {
        let (tx, rx) = chan();
        let res = Arc::new(Mutex::new(Vec::new()));
        let val = 1729;

        use Ordering::*;
        let res_t = res.clone();
        let h = tokio::spawn(async move {
            res_t.lock().unwrap().push(Sending(val));
            tx.send(val).await;
            res_t.lock().unwrap().push(Sent);
        });

        let res_r = res.clone();
        let k = tokio::spawn(async move {
            sleep(Duration::from_millis(100)).await;
            res_r.lock().unwrap().push(Receiving);
            let val = rx.await;
            res_r.lock().unwrap().push(Received(val));
        });

        let _ = tokio::join!(h, k);

        let res = res.lock().unwrap();
        // tokio: let target = vec![Sending(val), Receiving, Received(val), Sent];
        let target = vec![Sending(val), Receiving, Sent, Received(val)];
        assert!(res.as_slice() == target.as_slice());
    }
}
