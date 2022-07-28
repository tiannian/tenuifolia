use libp2p::identity::{ed25519, secp256k1, Keypair};

use crate::Result;

pub enum KeypairType {
    Ed25519,
    Secp256k1,
}

pub struct PeerKeypair {
    pub ty: KeypairType,
    pub secret_key: Vec<u8>,
}

impl PeerKeypair {
    pub(crate) fn to_libp2p_keypair(&self) -> Result<Keypair> {
        Ok(match self.ty {
            KeypairType::Ed25519 => {
                let sk = ed25519::SecretKey::from_bytes(self.secret_key.clone())?;
                Keypair::Ed25519(ed25519::Keypair::from(sk))
            }
            KeypairType::Secp256k1 => {
                let sk = secp256k1::SecretKey::from_bytes(self.secret_key.clone())?;
                Keypair::Secp256k1(secp256k1::Keypair::from(sk))
            }
        })
    }
}

pub struct PeerKeys {
    pub peer_id: Vec<u8>,
    pub keypairs: Vec<PeerKeypair>,
}
