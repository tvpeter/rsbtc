use std::fmt::{self, write};

use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::U256;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Hash(U256);

impl Hash {
    pub fn hash<T: serde::Serialize>(data: &T) -> Self {
        let mut serialized: Vec<u8> = vec![];

        if let Err(e) = ciborium::into_writer(data, &mut serialized) {
            panic!("failed to serialize data: {:?}", e);
        }

        let hash = digest(&serialized);

        let hash_bytes = hex::decode(hash).unwrap();
        let hash_array: [u8; 32] = hash_bytes.as_slice().try_into().unwrap();

        let u256_hash = U256::from_big_endian(&hash_array);
        Hash(u256_hash)
    }

    pub fn matches_target(&self, target: U256) -> bool {
        self.0 <= target
    }
    /// zero hash
    pub fn zero() -> Self {
        Hash(U256::zero())
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
