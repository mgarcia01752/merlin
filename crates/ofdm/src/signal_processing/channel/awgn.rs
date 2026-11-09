/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent seeded sample-domain AWGN; unitary FFT preserves complex variance.
use crate::{
    error::invalid,
    signal_processing::{noise::awgn::add_awgn, waveform::SampleBlock},
    SimulationError,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Owns a noise-only stream; payload/source RNGs are never shared.
pub struct SampleAwgn {
    power: f64,
    next_sample: u64,
    rng: ChaCha8Rng,
}
impl SampleAwgn {
    /// Starts a contiguous complex sample-noise stream.
    /// # Arguments
    /// * `power` - E[|n|²] in normalized complex sample units; zero disables draws.
    /// * `seed` - Reproducible noise-only seed.
    /// * `first_sample` - Expected first absolute input tick.
    /// # Returns
    /// Independent bounded-memory channel state.
    /// # Errors
    /// Rejects negative, nonfinite or excessive normalized noise power.
    pub fn new(power: f64, seed: u64, first_sample: u64) -> Result<Self, SimulationError> {
        if !power.is_finite() || !(0.0..=super::MAX_NORMALIZED_NOISE_POWER).contains(&power) {
            return Err(invalid("Invalid sample AWGN power."));
        }
        Ok(Self {
            power,
            next_sample: first_sample,
            rng: ChaCha8Rng::seed_from_u64(seed),
        })
    }
    /// Adds noise without changing the original block or its absolute indices.
    /// # Arguments
    /// * `input` - Contiguous bounded samples at the configured unitary reference.
    /// # Returns
    /// Owned noisy samples; chunk boundaries do not change the random sequence.
    /// # Errors
    /// Rejects discontinuity or numeric overflow without committing RNG/clock state.
    pub fn transfer(&mut self, input: &SampleBlock) -> Result<SampleBlock, SimulationError> {
        if input.start_index() != self.next_sample {
            return Err(invalid("AWGN sample stream is discontinuous."));
        }
        let mut rng = self.rng.clone();
        let mut samples = input.samples().to_vec();
        if self.power > 0.0 {
            for point in &mut samples {
                add_awgn(point, self.power, &mut rng);
            }
        }
        let output = SampleBlock::new(input.start_index(), samples)?;
        self.rng = rng;
        self.next_sample = output.end_index();
        Ok(output)
    }
}
