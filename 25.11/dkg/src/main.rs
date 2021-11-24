use std::sync::{Arc, Mutex};

use crypto::threshold_signatures::KeyBox;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

#[derive(Clone, Copy)]
enum Transaction {
    // IMPLEMENTME
    Noop,
}

#[derive(Clone)]
struct Block {
    txs: Vec<Transaction>,
}

#[derive(Clone)]
struct Blockchain {
    n_authorities: u64,
    tx_pool: Arc<Mutex<Vec<Transaction>>>,
    subscribers: Vec<UnboundedSender<Block>>,
}

type Authority = u64;

impl Blockchain {
    fn new(n_authorities: u64) -> Self {
        Self {
            n_authorities,
            tx_pool: Arc::new(Mutex::new(Vec::new())),
            subscribers: Vec::new(),
        }
    }
    fn post(&self, tx: Transaction) {
        let mut tx_pool = self.tx_pool.lock().unwrap();
        tx_pool.push(tx);
        if tx_pool.len() == self.n_authorities as usize {
            let txs = tx_pool.drain(..).collect();
            let block = Block { txs };
            self.subscribers
                .iter()
                .for_each(|s| s.send(block.clone()).map_err(|_| ()).unwrap());
        }
    }
    fn subscribe(&mut self, subscriber: UnboundedSender<Block>) {
        self.subscribers.push(subscriber)
    }
}

struct Dkg {
    _auth: Authority,
    _threshold: u64,
    blockchain: Blockchain,
    next_block: UnboundedReceiver<Block>,
}

// IMPLEMENTME
impl Dkg {
    // Round 0: posting encryption public keys
    async fn round_0(&mut self) {
        self.blockchain.post(Transaction::Noop);
        let _block = self.next_block.recv().await;
        // extract data and perform necessary computation
    }

    // Round 1: sharing phase
    async fn round_1(&mut self) {
        self.blockchain.post(Transaction::Noop);
        let _block = self.next_block.recv().await;
        // extract data and perform necessary computation
    }

    // Round 2: dispute phase
    async fn round_2(&mut self) {
        self.blockchain.post(Transaction::Noop);
        let _block = self.next_block.recv().await;
        // extract data and perform necessary computation
    }

    // Round 3: key derivation
    fn round_3(&mut self) -> KeyBox {
        // extract data and perform necessary computation
        KeyBox::default()
    }

    async fn protocol(mut self) -> KeyBox {
        self.round_0().await;
        self.round_1().await;
        self.round_2().await;
        self.round_3()
    }
}

#[tokio::main]
async fn main() {
    let n_authorities = 3;
    let threshold = 2;
    let mut blockchain = Blockchain::new(n_authorities);
    let mut handles = Vec::new();

    for auth in 0..n_authorities {
        let (subscriber, next_block) = mpsc::unbounded_channel();
        blockchain.subscribe(subscriber);

        let dkg = Dkg {
            _auth: auth,
            _threshold: threshold,
            blockchain: blockchain.clone(),
            next_block,
        };

        handles.push(tokio::spawn(dkg.protocol()));
    }

    let mut kbs = Vec::new();
    for h in &mut handles {
        kbs.push(h.await.unwrap());
    }

    let msg = b"secret message";
    let shares = (0..threshold)
        .map(|id| kbs[id as usize].generate_share(&msg[..]))
        .collect::<Vec<_>>();

    let signature = kbs[0].combine_shares(&shares).unwrap();

    assert!(kbs[0].verify_signature(&msg[..], &signature));
}
