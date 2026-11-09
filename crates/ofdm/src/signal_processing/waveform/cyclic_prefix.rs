/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! TX prefix insertion and RX removal are distinct pure operations on fixed geometry.
use super::{samples::validate_samples, WaveformGeometry};
use crate::{types::IqPair, SimulationError};
/// Prepends the selected trailing useful samples without modifying the input.
/// # Arguments
/// * `geometry` - Validated per-run prefix/FFT selection.
/// * `useful` - Exactly N finite complex time samples from the IFFT.
/// # Returns
/// N+CP samples: trailing CP values followed by the complete useful symbol.
/// # Errors
/// Rejects wrong input length or nonfinite samples before producing output.
pub fn insert(
    geometry: &WaveformGeometry,
    useful: &[IqPair],
) -> Result<Vec<IqPair>, SimulationError> {
    validate_samples(useful, geometry.fft_samples())?;
    let mut output = Vec::with_capacity(geometry.stride_samples());
    output.extend_from_slice(&useful[useful.len() - geometry.prefix_samples()..]);
    output.extend_from_slice(useful);
    Ok(output)
}
/// Selects the useful FFT input from a correctly framed received symbol period.
/// # Arguments
/// * `geometry` - RX-owned validated prefix and FFT length.
/// * `framed` - Exactly N+CP received samples at the known symbol start.
/// # Returns
/// Borrowed N-sample useful window; this operation does not acquire timing.
/// # Errors
/// Rejects incomplete/oversized/nonfinite periods; never fills missing samples.
pub fn remove<'samples>(
    geometry: &WaveformGeometry,
    framed: &'samples [IqPair],
) -> Result<&'samples [IqPair], SimulationError> {
    validate_samples(framed, geometry.stride_samples())?;
    Ok(&framed[geometry.prefix_samples()..])
}
