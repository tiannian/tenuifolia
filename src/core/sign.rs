use digest::{Digest, Output};

pub struct Signature {
    pub set_index: i32,
    pub public_key_index: i32,
    pub signature: Vec<u8>,
}

pub struct Signatures {
    pub signatures: Vec<Signature>,
}

pub enum PublicKey {
    Ed25519(Vec<u8>),
    Secp256k1(Vec<u8>),
}

impl PublicKey {
    pub fn hash<D: Digest>(&self) -> Output<D> {
        match self {
            PublicKey::Ed25519(k) => D::digest(k),
            PublicKey::Secp256k1(k) => D::digest(k),
        }
    }
}
