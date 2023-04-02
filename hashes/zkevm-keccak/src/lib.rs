//! The zkEVM keccak circuit implementation, with some minor modifications
//! Credit goes to https://github.com/privacy-scaling-explorations/zkevm-circuits/tree/main/zkevm-circuits/src/keccak_circuit

#[cfg(feature = "halo2-axiom")]
/// Keccak packed multi
pub mod keccak_packed_multi;
#[cfg(feature = "halo2-axiom")]
/// Util
pub mod util;
#[cfg(feature = "halo2-axiom")]
pub use keccak_packed_multi::KeccakCircuitConfig as KeccakConfig;
