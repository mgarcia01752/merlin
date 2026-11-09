/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual PLC configuration, data/NCP transport and payload recovery through waveform samples.
use docsis_ofdm_fec::{
    simulation::{
        payload::reference::{run_payload_waveform, PayloadWaveformExperiment},
        test_points::{ProbeEvent, ProbeObserver, TestPointProbe},
    },
    transmission::{
        layout::CarrierLayoutSettings, packing::NcpModulation, SubcarrierSpacing,
        TransmissionSettings,
    },
    LabError,
};
struct UnusedObserver;
impl ProbeObserver for UnusedObserver {
    /// Receives no events from the example's disabled selector.
    /// # Arguments
    /// * `_event` - Read-only bounded observation if enabled by a consumer.
    /// # Returns
    /// Unit without introducing console output inside DSP.
    fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), LabError> {
        Ok(())
    }
}
/// Runs bounded actual payload waveform verification at both spacings.
/// # Returns
/// JSON with recovered payload errors and genuine control/payload decoder iterations.
/// # Errors
/// Propagates configuration, stage, FEC/CRC or JSON failures.
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
            time_interleaver_depth: spacing.max_time_interleaver_depth(),
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
        let experiment = PayloadWaveformExperiment {
            settings,
            layout,
            ncp_modulation: NcpModulation::Qpsk,
            source_symbols: 160,
            payload_at_symbols: vec![0, 61, 126, 140],
            seed: 123,
            receive_block_samples: 137,
        };
        results.push(run_payload_waveform(
            &experiment,
            &mut TestPointProbe::disabled(),
            &mut UnusedObserver,
        )?);
    }
    println!("{}", serde_json::to_string_pretty(&results)?);
    Ok(())
}
