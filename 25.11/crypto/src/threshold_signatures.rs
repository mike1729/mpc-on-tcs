use bls12_381::{G1Affine, Scalar};
use sha3::{Digest, Sha3_256};

#[derive(Clone, Default)]
pub struct Signature {}

#[derive(Clone, Default)]
pub struct VerifyKey {}

pub type Message = [u8];

impl VerifyKey {
    pub fn verify(&self, _msg: &Message, _sgn: &Signature) -> bool {
        false
    }

    pub fn from_secret(_secret: Scalar) -> Self {
        Default::default()
    }
}

#[derive(Clone, Default)]
pub struct ShareProvider {}

impl ShareProvider {
    pub fn generate(_id: u64) -> Self {
        Default::default()
    }

    pub fn from_secret(_id: u64, _secret: Scalar) -> Self {
        Default::default()
    }

    pub fn sign(&self, _msg: &Message) -> Signature {
        Default::default()
    }

    pub fn verify(&self, _msg: &Message, _sgn: &Signature) -> bool {
        false
    }

    pub fn verify_key(&self) -> VerifyKey {
        Default::default()
    }
}

#[derive(Clone, Default)]
pub struct Share {}

#[derive(Clone, Default)]
pub struct KeyBox {}

impl KeyBox {
    pub fn new(
        _share_provider: ShareProvider,
        _verify_keys: Vec<VerifyKey>,
        _master_key: VerifyKey,
        _threshold: u64,
    ) -> Self {
        Default::default()
    }

    pub fn generate_share(&self, _msg: &Message) -> Share {
        Default::default()
    }

    pub fn verify_share(&self, _msg: &Message, _share: &Share) -> bool {
        false
    }

    pub fn combine_shares(&self, _shares: &[Share]) -> Option<Signature> {
        None
    }

    pub fn verify_signature(&self, _msg: &Message, _sgn: &Signature) -> bool {
        false
    }

    pub fn n_parties(&self) -> u64 {
        0
    }

    pub fn threshold(&self) -> u64 {
        0
    }
}

fn _lagrange_coef(_knots: impl Iterator<Item = Scalar>, _knot: Scalar, _target: Scalar) -> Scalar {
    Default::default()
}

// WARNING: this hashing function gen ^ hash(msg) is not secure because the log of the hash is known.
pub fn hash_to_curve(msg: &Message) -> G1Affine {
    let data = Sha3_256::digest(msg);

    let mut scalar_raw = [0u64; 4];
    for i in 0usize..4 {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data[i * 8..(i + 1) * 8]);
        scalar_raw[i] = u64::from_le_bytes(bytes);
    }

    let scalar = Scalar::from_raw(scalar_raw);

    G1Affine::from(G1Affine::generator() * scalar)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn random_scalar() -> Scalar {
        let mut rng = rand::thread_rng();
        Scalar::from_raw([rng.gen(), rng.gen(), rng.gen(), rng.gen()])
    }

    fn random_msg() -> Vec<u8> {
        rand::thread_rng().gen::<[u8; 32]>().to_vec()
    }

    fn poly_eval(coeffs: &[Scalar], x: Scalar) -> Scalar {
        let mut eval = Scalar::zero();
        for coeff in coeffs.iter().rev() {
            eval *= x;
            eval += coeff;
        }

        eval
    }

    fn generate_threshold_pairs(n_members: u64, threshold: u64) -> (Vec<ShareProvider>, VerifyKey) {
        assert!(n_members >= threshold && threshold > 0);

        let coeffs = (0..threshold).map(|_| random_scalar()).collect::<Vec<_>>();

        let secret = coeffs[0];
        let master_key = VerifyKey::from_secret(secret);

        let pairs = (0..n_members)
            .map(|i| {
                let x = Scalar::from(i + 1);
                let secret = poly_eval(&coeffs, x);
                ShareProvider::from_secret(i, secret)
            })
            .collect::<Vec<_>>();

        (pairs, master_key)
    }

    #[test]
    fn correct_sign() {
        let pair = ShareProvider::generate(7);
        let msg = random_msg();
        let sgn = pair.sign(&msg);

        assert!(pair.verify(&msg, &sgn));
    }

    #[test]
    fn combine_shares() {
        let (n_members, threshold) = (3u64, 2u64);
        let (share_providers, master_key) = generate_threshold_pairs(n_members, threshold);
        let verifiers = (0..n_members)
            .map(|id| share_providers[id as usize].verify_key())
            .collect::<Vec<_>>();

        let kbs = (0..n_members)
            .map(|id| {
                KeyBox::new(
                    share_providers[id as usize].clone(),
                    verifiers.clone(),
                    master_key.clone(),
                    threshold,
                )
            })
            .collect::<Vec<_>>();

        let msg = random_msg();
        let shares = (0..threshold)
            .map(|id| kbs[id as usize].generate_share(&msg))
            .collect::<Vec<_>>();

        let signature = kbs[0].combine_shares(&shares).unwrap();

        assert!(kbs[0].verify_signature(&msg, &signature));
    }
}
