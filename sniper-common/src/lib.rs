pub mod service;

#[cfg(feature = "client")]
pub mod client;

pub const SOCKET_PATH: &str = "/tmp/sniper.socket";

pub use dashmap;
pub use qp_trie::Trie;
pub use rayon;
