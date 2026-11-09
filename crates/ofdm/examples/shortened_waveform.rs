/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit shortened128-QAM transmission through acquired PLC/NCP and waveform samples.
use docsis_ofdm_fec::{
    simulation::{
        payload::shortened::run_shortened_waveform,
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::{layout::CarrierLayoutSettings, TransmissionSettings},
};
/// Prints a bounded noiseless reference result; no input/output files are accessed.
/// # Returns
/// JSON describing independent original-bit errors and actual FEC/link outcomes.
/// # Errors
/// Propagates setup, control acquisition, stage or serialization failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = TransmissionSettings {
        occupied_bandwidth_hz: 24_000_000,
        time_interleaver_depth: 4,
        ..Default::default()
    };
    let layout = CarrierLayoutSettings {
        zero_frequency_hz: 100_000_000,
        first_active_bin: 320,
        active_end_exclusive: 1280,
        plc_first_bin: 752,
        excluded_bins: vec![1200],
        additional_continuous_pilots: vec![352, 448, 544, 920, 1008, 1080, 1176, 1256],
    };
    let payloads: Vec<_> = [1032, 6136]
        .iter()
        .map(|count| {
            (0..*count)
                .map(|index| u8::from((index * 17 + index / 11) % 19 < 9))
                .collect()
        })
        .collect();
    let result = run_shortened_waveform(
        &settings,
        &layout,
        &vec![7; settings.subcarrier_spacing.fft_samples() as usize],
        &payloads,
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
    )?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    if result.words.iter().any(|word| {
        word.bit_errors != 0
            || !word.ldpc_success
            || !word.bch_success
            || !word.known_zeros_consistent
    }) {
        return Err("Shortened reference recovery failed.".into());
    }
    Ok(())
}
