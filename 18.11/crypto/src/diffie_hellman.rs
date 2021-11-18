use bls12_381::Scalar;
use sha3::{Digest, Sha3_256};

#[derive(Default)]
pub struct EncryptionPublicKey {}

impl EncryptionPublicKey {
    pub fn from_scalar(_scalar: Scalar) -> Self {
        Default::default()
    }

    pub fn to_encryption_key(&self, _other: Self) -> EncryptionKey {
        Default::default()
    }
}

#[derive(Default)]
pub struct EncryptionKey {}

pub type EncryptedShare = [u8; 32];

impl EncryptionKey {
    pub fn encrypt(&self, _scalar: &Scalar) -> EncryptedShare {
        Default::default()
    }

    pub fn decrypt(&self, _encrypted: &EncryptedShare) -> Option<Scalar> {
        None
    }

    pub fn is_correct(&self, _epk1: &EncryptionPublicKey, _ek2: &EncryptionPublicKey) -> bool {
        true
    }
}

pub fn hash(x: &[u8]) -> [u8; 32] {
    Sha3_256::digest(x).into()
}
