pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;
pub mod error;


use serde::{Deserialize, Serialize};
use uint::construct_uint;

construct_uint! {
    // Construct an unsigned 256-bit integer
    // consisting of 4 x 64-bit words
    #[derive(Serialize, Deserialize)]
    pub struct U256(4);
}
