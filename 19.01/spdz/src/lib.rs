use crossbeam::channel::{bounded, Receiver, Sender};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Network {
    n_parties: usize,
    txs: Vec<Sender<u128>>,
    rxs: Vec<Receiver<u128>>,
}

impl Network {
    pub fn new(n_parties: usize) -> (Self) {
        let mut rxs = Vec::new();
        let mut txs = Vec::new();
        (0..n_parties).for_each(|_| {
            let (tx, rx) = bounded(n_parties);
            txs.push(tx);
            rxs.push(rx);
        });
        Self {
            n_parties,
            txs,
            rxs,
        }
    }

    pub fn send(&mut self, share: u128) {
        (0..self.n_parties).for_each(|i| self.txs[i].send(share).expect("channel is closed"));
    }
    pub fn recv(&mut self) -> Vec<u128> {
        (0..self.n_parties)
            .map(|i| self.rxs[i].recv())
            .map(Result::unwrap)
            .collect()
    }
}

pub struct SPDZ {
    n_parties: usize,
    threshold: usize,
    inputs: HashMap<String, u128>,
    net: Network,
}

impl SPDZ {
    pub fn new(
        n_parties: usize,
        threshold: usize,
        inputs: HashMap<String, u128>,
        net: Network,
    ) -> Self {
        Self {
            n_parties,
            threshold,
            inputs,
            net,
        }
    }

    pub fn setup(mut self, n_mul: usize) -> Self {
        self
    }

    pub fn add(self: &mut Self, x: &str, y: &str, z: &str) {}

    pub fn mul_by_const(self: &mut Self, x: &str, c: u128, z: &str) {}

    pub fn mul(self: &mut Self, x: &str, y: &str, z: &str) {}

    pub fn reveal(self: &Self, x: &str) -> u128 {
        0
    }
}

#[cfg(test)]
mod tests {

    use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

    use super::*;
    const N_PARTIES: usize = 5;
    const THRESHOLD: usize = 2;
    use std::iter;

    fn gen(n_mul: usize) -> Vec<SPDZ> {
        let net = Network::new(N_PARTIES);

        (0..N_PARTIES)
            .map(|i| {
                let inputs = iter::once(format!("x{}", i))
                    .zip(iter::once(i as u128))
                    .collect();

                SPDZ::new(N_PARTIES, THRESHOLD, inputs, net.clone())
            })
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|spdz| spdz.setup(n_mul))
            .collect()
    }

    fn reveal_and_check(spdzs: Vec<SPDZ>, val: u128) -> bool {
        spdzs[..THRESHOLD]
            .par_iter()
            .map(|spdz| spdz.reveal("z"))
            .collect::<Vec<_>>()
            .into_iter()
            .all(|res| res == val)
    }

    #[test]
    fn tadd() {
        let mut spdzs = gen(0);
        spdzs
            .iter_mut()
            .take(THRESHOLD)
            .for_each(|spdz| spdz.add("x3", "x4", "z"));
        assert!(reveal_and_check(spdzs, 7));
    }

    #[test]
    fn tmul_by_const() {
        let mut spdzs = gen(0);
        spdzs
            .iter_mut()
            .take(THRESHOLD)
            .for_each(|spdz| spdz.mul_by_const("x4", 2, "z"));
        assert!(reveal_and_check(spdzs, 8));
    }

    #[test]
    fn tmul() {
        let mut spdzs = gen(0);
        spdzs
            .iter_mut()
            .take(THRESHOLD)
            .for_each(|spdz| spdz.mul("x3", "x4", "z"));
        assert!(reveal_and_check(spdzs, 12));
    }
}
