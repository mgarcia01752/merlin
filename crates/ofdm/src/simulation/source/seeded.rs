/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Existing reproducible random payload source; this is not a PRBS definition.
use super::SimulationSeed;
use fec_codec::parameters::BCH_INPUT_BITS;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
/// Job-owned seeded stream; the runner shares it with AWGN to preserve sequences.
pub struct SeededSource {
    rng: ChaCha8Rng,
}
impl SeededSource {
    /// Initializes a deterministic, noncryptographic source stream.
    /// # Arguments
    /// * `seed` - Reproducible job seed, without hidden entropy.
    /// # Returns
    /// Source state for repeated current-mode payload blocks.
    pub fn new(seed: SimulationSeed) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
        }
    }
    /// Generates one current-mode BCH information block.
    /// # Returns
    /// Exactly14232 binary bits. This is seeded random data, not a PRBS sequence.
    pub fn next_payload(&mut self) -> Vec<u8> {
        (0..BCH_INPUT_BITS)
            .map(|_| u8::from(self.rng.random::<bool>()))
            .collect()
    }
    /// Borrows the same job stream for channel draws between payload blocks.
    /// # Returns
    /// Mutable generator, keeping the established source/channel draw order.
    pub(crate) fn rng_mut(&mut self) -> &mut ChaCha8Rng {
        &mut self.rng
    }
}
