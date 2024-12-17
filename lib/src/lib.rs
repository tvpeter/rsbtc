pub mod crypto;
pub mod sha256;
pub mod types;
pub mod util;

use uint::construct_uint;

construct_uint! {
    // Construct an unsigned 256-bit integer
    // consisting of 4 x 64-bit words
    pub struct U256(4);
}
