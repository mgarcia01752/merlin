/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded known-reference waveform demonstration with actual PLC messages at both spacings.
use docsis_ofdm_fec::{
    simulation::{
        test_points::{ProbeEvent, ProbeObserver, TestPointProbe},
        waveform::reference::{run_reference_waveform, ReferenceExperiment},
    },
    transmission::{layout::CarrierLayoutSettings, SubcarrierSpacing, TransmissionSettings},
    SimulationError,
};
struct UnusedObserver;
impl ProbeObserver for UnusedObserver {
    /// Accepts no data when the example's probe is disabled.
    /// # Arguments
    /// * `_event` - Unused bounded event if observation is enabled by a caller.
    /// # Returns
    /// Unit without printing inside scientific processing.
    fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
        Ok(())
    }
}
/// Reports compact results from two PLC frames through each waveform geometry.
/// # Returns
/// Unit after printing JSON containing numerical and actual decoding comparisons.
/// # Errors
/// Propagates configuration, processing, CRC/FEC or serialization errors.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let divisor = if spacing == SubcarrierSpacing::Khz25 {
            1
        } else {
            2
        };
        let settings = TransmissionSettings {
            occupied_bandwidth_hz: 24_000_000,
            subcarrier_spacing: spacing,
            ..Default::default()
        };
        let layout = CarrierLayoutSettings {
            zero_frequency_hz: 100_000_000,
            first_active_bin: 320 / divisor,
            active_end_exclusive: 1280 / divisor,
            plc_first_bin: 752 / divisor,
            excluded_bins: vec![1200 / divisor],
            additional_continuous_pilots: [352, 448, 544, 920, 1008, 1080, 1176, 1256]
                .map(|bin| bin / divisor)
                .to_vec(),
        };
        let experiment = ReferenceExperiment {
            settings,
            layout,
            frames: 2,
            receive_block_samples: 137,
            first_sample: 0,
            reference_rms: 1.0,
        };
        results.push(run_reference_waveform(
            &experiment,
            &mut TestPointProbe::disabled(),
            &mut UnusedObserver,
        )?);
    }
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
