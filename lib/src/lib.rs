pub mod crypto;
pub mod error;
pub mod sha256;
pub mod types;
pub mod util;

use serde::{Deserialize, Serialize};
use uint::construct_uint;

construct_uint! {
    // Construct an unsigned 256-bit integer
    // consisting of 4 x 64-bit words
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}

// initial reward in bitcoin - multiply by 10^8 to get satoshis
pub const INITIAL_REWARD: u64 = 50;
// halving interval in blocks
pub const HALVING_INTERVAL: u64 = 210;
// ideal block time in seconds
