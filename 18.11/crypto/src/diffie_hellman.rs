use bls12_381::Scalar;
use sha3::{Digest, Sha3_256};

#[derive(Default)]
pub struct EncryptionPublicKey {}

impl EncryptionPublicKey {
    pub fn from_scalar(_scalar: Scalar) -> Self {
        Default::default()
    }

    pub fn to_encryption_key(&self, _scalar: Scalar) -> EncryptionKey {
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
        false
    }
}

pub fn hash(x: &[u8]) -> [u8; 32] {
    Sha3_256::digest(x).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt() {
        let enckey = EncryptionKey {};
        let scalar = Scalar::from_raw([1729, 2137, 1994, 1500]);
        let enc = enckey.encrypt(&scalar);
        let dec = enckey.decrypt(&enc);

        assert!(dec.is_some() && dec.unwrap() == scalar);
    }

    #[test]
    fn correctness() {
        let s1 = Scalar::one();
        let s2 = Scalar::one().double();
        let epk1 = EncryptionPublicKey::from_scalar(s1);
        let epk2 = EncryptionPublicKey::from_scalar(s2);
        let enckey = epk1.to_encryption_key(s2);

        assert!(enckey.is_correct(&epk1, &epk2));
    }
}
