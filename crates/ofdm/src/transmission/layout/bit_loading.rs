/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 section7.5.5.5.1, PDF126–127: Ci=P^-1(Bi), independent of time phase.
use crate::{
    error::invalid,
    signal_processing::interleaving::FrequencyInterleaver,
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
    SimulationError,
};
/// Converts a full transmitted-frequency profile into pre-interleaving NI order.
/// # Arguments
/// * `layout` - Validated protected roles and physical frequency ordering.
/// * `physical_loading` - Full FFT data bits; protected bins are overridden by layout.
/// # Returns
/// One bit count per source rank, including locations used by scattered pilots.
/// # Errors
/// Rejects wrong shape, invalid data loading, or invalid permutation geometry.
pub fn source_bit_loading(
    layout: &CarrierLayout,
    physical_loading: &[u8],
) -> Result<Vec<u8>, SimulationError> {
    if physical_loading.len() != layout.static_roles().len() {
        return Err(invalid(
            "Data profile must cover the full physical FFT grid.",
        ));
    }
    let mut reduced = Vec::new();
    for (bits, role) in physical_loading.iter().zip(layout.static_roles()) {
        if *role == PhysicalCarrierRole::Data {
            if !matches!(bits, 0 | 4 | 6..=14) {
                return Err(invalid("Invalid data profile bit loading."));
            }
            reduced.push(*bits);
        }
    }
    FrequencyInterleaver::new(layout.subcarrier_spacing(), reduced.len())?.deinterleave(&reduced)
}
