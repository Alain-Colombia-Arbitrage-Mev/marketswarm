pub mod analysis;
pub mod connectors;
pub mod evaluation;
pub mod inference;
pub mod runtime;
pub mod schema;
pub mod store;
pub mod wtp;

pub fn digest(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}
