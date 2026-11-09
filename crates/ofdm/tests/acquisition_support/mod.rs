/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Edge-excluded PLC acquisition fixtures.
use docsis_ofdm_fec::transmission::{
    layout::CarrierLayoutSettings, SubcarrierSpacing, TransmissionSettings,
};
/// Supplies a specification-compatible edge-excluded layout for PLC acquisition tests.
/// # Arguments
/// * `spacing` - Fixed25or50kHz spacing.
/// * `fragmented` - Adds bounded isolated exclusions to force MC frame segmentation.
/// # Returns
/// Fixed settings and raw layout used by both descriptor and full-signal tests.
pub fn acquisition_setup(
    spacing: SubcarrierSpacing,
    fragmented: bool,
) -> (TransmissionSettings, CarrierLayoutSettings) {
    let divisor = match spacing {
        SubcarrierSpacing::Khz25 => 1,
        SubcarrierSpacing::Khz50 => 2,
    };
    let settings = TransmissionSettings {
        occupied_bandwidth_hz: 24_000_000,
        subcarrier_spacing: spacing,
        ..Default::default()
    };
    let mut layout = CarrierLayoutSettings {
        zero_frequency_hz: 100_000_000,
        first_active_bin: 320 / divisor,
        active_end_exclusive: 1280 / divisor,
        plc_first_bin: 752 / divisor,
        excluded_bins: vec![1200 / divisor],
        additional_continuous_pilots: [352, 448, 544, 920, 1008, 1080, 1176, 1256]
            .map(|bin| bin / divisor)
            .to_vec(),
    };
    if fragmented {
        for bin in (layout.first_active_bin + 2..layout.plc_first_bin - 112 / divisor)
            .step_by(3)
            .filter(|bin| !layout.additional_continuous_pilots.contains(bin))
            .take(40)
        {
            layout.excluded_bins.push(bin);
        }
    }
    (settings, layout)
}
