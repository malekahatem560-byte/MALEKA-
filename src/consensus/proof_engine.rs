use sha2::{Sha256, Digest};
use ed25519_dalek::{VerifyingKey, Signature, Signer, SigningKey};
use rand::rngs::OsRng;

pub struct ProofArtifact {
    pub theory_hash: String,
    pub public_key: String,
    pub signature: String,
    pub timestamp: i64,
}

impl ProofArtifact {
    pub fn new(data: &str, signing_key: &SigningKey) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        let theory_hash = format!("{:x}", result);

        let signature = signing_key.sign(data.as_bytes());
        let signature_bytes = signature.to_bytes();
        let signature_hex = hex::encode(signature_bytes);

        Self {
            theory_hash,
            public_key: hex::encode(signing_key.verifying_key().to_bytes()),
            signature: signature_hex,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn verify(&self, data: &str) -> bool {
        if self.theory_hash.is_empty() || self.signature.is_empty() {
            return false;
        }
        let mut hasher = Sha256::new();
        hasher.update(data);
        let computed_hash = format!("{:x}", hasher.finalize());
        if computed_hash != self.theory_hash {
            return false;
        }
        !self.public_key.is_empty()
    }
}

pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}
