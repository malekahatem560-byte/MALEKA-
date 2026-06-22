use uuid::Uuid;
use chrono::Utc;

pub struct IdentityKernel {
    pub civilization_id: Uuid,
    pub public_key: String,
    pub constitution_hash: String,
    pub creation_epoch: i64,
}

impl IdentityKernel {
    pub fn new(constitution_hash: String, public_key: String) -> Self {
        Self {
            civilization_id: Uuid::new_v4(),
            public_key,
            constitution_hash,
            creation_epoch: Utc::now().timestamp(),
        }
    }

    pub fn verify_sovereignty(&self) -> bool {
        !self.constitution_hash.is_empty()
    }
}
