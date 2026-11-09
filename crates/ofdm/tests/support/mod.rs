/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit24MHz geometry for cell-interleaving tests; additional pilots supplied.
use docsis_ofdm_fec::transmission::{
    layout::{CarrierLayout, CarrierLayoutSettings},
    SubcarrierSpacing, TransmissionSettings,
};

/// Builds a bounded physical geometry for one spacing, with fixed protected roles.
/// # Arguments
/// * `spacing` -25or50kHz, keeping the same24MHz physical envelope.
/// # Returns
/// Validated layout; additional pilot positions are synthetic, not auto-generated.
pub fn layout(spacing: SubcarrierSpacing) -> CarrierLayout {
    let divisor = match spacing {
        SubcarrierSpacing::Khz25 => 1,
        SubcarrierSpacing::Khz50 => 2,
    };
    let transmission = TransmissionSettings {
        occupied_bandwidth_hz: 24_000_000,
        subcarrier_spacing: spacing,
        ..Default::default()
    };
    CarrierLayout::new(
        &transmission,
        &CarrierLayoutSettings {
            zero_frequency_hz: 100_000_000,
            first_active_bin: 0,
            active_end_exclusive: 960 / divisor,
            plc_first_bin: 432 / divisor,
            excluded_bins: vec![800 / divisor],
            additional_continuous_pilots: [32, 128, 224, 600, 688, 760, 856, 936]
                .map(|bin| bin / divisor)
                .to_vec(),
        },
    )
    .unwrap()
}
