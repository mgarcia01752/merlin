/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Pure upstream trailing prefix and leading useful-symbol suffix.
use crate::{configuration::Waveform, error::invalid, types::Iq, PhyResult};

/// Extends one useful `symbol` using accepted CP/RP `geometry`.
///
/// Returns x[N-C..N] + x + x[0..R], preserving inputs and sample order.
/// Rejects wrong size/nonfinite values before allocating output.
pub fn cyclic_extension(geometry: &Waveform, symbol: &[Iq]) -> PhyResult<Vec<Iq>> {
    let n = geometry.numerology().fft_len();
    if symbol.len() != n || symbol.iter().any(|z| !z.is_finite()) {
        return Err(invalid("cyclic_extension.input"));
    }
    Ok(symbol[n - geometry.prefix()..]
        .iter()
        .chain(symbol)
        .chain(&symbol[..geometry.rolloff()])
        .copied()
        .collect())
}
