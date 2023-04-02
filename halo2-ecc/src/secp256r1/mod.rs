use crate::halo2_proofs::halo2curves::secp256r1::Fp;

use crate::ecc;
use crate::fields::fp;

#[allow(dead_code)]
type FpChip<F> = fp::FpConfig<F, Fp>;
#[allow(dead_code)]
type Secp256r1Chip<F> = ecc::EccChip<F, FpChip<F>>;
#[allow(dead_code)]
const SECP_B: u64 = 7;

#[cfg(test)]
mod tests;
